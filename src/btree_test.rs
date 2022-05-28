use std::mem::size_of;

use serde::{de::DeserializeOwned, Deserialize, Serialize, Serializer};

use crate::{
    block_dev::BlockDevice,
    btree::{AllocRec, BtreeBlock},
    file_blk::FileBlockDevice,
};

pub struct BtreeOperator<TKey, TVal> {
    dev: Box<dyn BlockDevice>,
    btroot_block: u64,
    __type_key: std::marker::PhantomData<TKey>,
    __type_val: std::marker::PhantomData<TVal>,
}
impl<'de, TKey, TVal> BtreeOperator<TKey, TVal>
where
    TKey: DeserializeOwned + PartialOrd,
    TVal: DeserializeOwned + Serialize,
{
    pub fn new(dev: Box<dyn BlockDevice>, btroot_block: u64) -> Self {
        BtreeOperator {
            dev,
            btroot_block,
            __type_key: std::marker::PhantomData,
            __type_val: std::marker::PhantomData,
        }
    }

    pub fn create_btree(&self) -> () {
        let new_node = BtreeBlock::new(0);
        let buf_node = bincode::serialize(&new_node).unwrap();
        self.dev.write_block(new_node.blkno as usize, &buf_node);
    }

    pub fn get<'a>(&self, key: &TKey) -> Option<TVal> {
        let mut buf = [0u8; 512];
        // 读一个 block
        self.dev.read_block(self.btroot_block as usize, &mut buf);
        // -- node 为 block 的头部
        let node: BtreeBlock = bincode::deserialize(&buf).unwrap();
        // -- recs 为 block 的剩余部分
        let opt = self.find_rec_in_block(&node, &buf, key);
        match opt {
            Some(rec) => {
                if rec.1 == *key {
                    Some(rec.2)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    pub fn set(&self, key: &TKey, value: &TVal) -> bool {
        let mut buf = [0u8; 512];
        // 读一个 block
        self.dev.read_block(self.btroot_block as usize, &mut buf);
        // -- node 为 block 的头部
        let mut header: BtreeBlock = bincode::deserialize(&buf).unwrap();
        // -- recs 为 block 的剩余部分
        let ret = self.find_rec_in_block(&header, &buf, key);
        // 无记录
        if ret.is_none() {
            // 追加到末尾
            let off_new_rec_opt = self.get_rec_off(4096, header.numrecs as usize);
            if off_new_rec_opt.is_none() {
                // TODO: 更复杂的 b+tree 操作
                return true;
            }
            let off_new_rec = off_new_rec_opt.unwrap();
            let rec_buf = bincode::serialize(value).unwrap();
            // update buf with new rec
            let s_rec = size_of::<TVal>();
            buf[off_new_rec as usize..off_new_rec + s_rec].copy_from_slice(&rec_buf);
            // update header
            header.numrecs += 1;
            let buf_hdr = bincode::serialize(&header).unwrap();
            buf[..size_of::<BtreeBlock>()].copy_from_slice(&buf_hdr);
            self.dev.write_block(self.btroot_block as usize, &buf);
            return true;
        } else {
            // 未找到
            let last_rec = ret.unwrap();
            if last_rec.1 == *key  {
                println!("found record");
                // TODO: update record with new value
            }
            // 可插入或追加
            if last_rec.0 <= header.numrecs - 1 {
                let rec_no = last_rec.0 + 1;
                let rec_off = self.get_rec_off(4096, rec_no as usize).unwrap();
                let rec_buf = bincode::serialize(value).unwrap();
                // update buf with new rec
                let s_rec = size_of::<TVal>();
                let n_rec_after = header.numrecs - rec_no;
                let s_rec_after = s_rec * n_rec_after as usize;
                // -- move recs after new rec
                // buf[rec_off as usize + s_rec..rec_off as usize + s_rec_after+ s_rec].copy_from_slice(&buf[rec_off as usize..rec_off as usize + s_rec_after]);
                buf.copy_within(
                    rec_off as usize..rec_off as usize + s_rec_after,
                    rec_off as usize + s_rec,
                );
                buf[rec_off as usize..rec_off + s_rec].copy_from_slice(&rec_buf);
                // update header
                header.numrecs += 1;
                let buf_hdr = bincode::serialize(&header).unwrap();
                buf[..size_of::<BtreeBlock>()].copy_from_slice(&buf_hdr);
                self.dev.write_block(self.btroot_block as usize, &buf);
                return true;
            } else {
                assert_eq!("", "unreachable");
            }

            return true;
        }
    }
    /// 获取第 rec_no 个记录的字节偏移
    pub fn get_rec_off(&self, blocksize: usize, rec_no: usize) -> Option<usize> {
        let sz_recs = size_of::<TVal>();
        let sz_header = size_of::<BtreeBlock>();
        let off_new_rec = sz_header + sz_recs * rec_no as usize;
        if off_new_rec > blocksize {
            return None;
        }
        return Some(off_new_rec);
    }
    /// 读取与 key 匹配的记录, 或者在 key 之前最近的记录. 如果未找到, 则返回 None
    pub fn find_rec_in_block(
        &self,
        header: &BtreeBlock,
        buf: &[u8],
        key: &TKey,
    ) -> Option<(u16, TKey, TVal)> {
        let n_recs = header.numrecs;
        if n_recs == 0 {
            return None;
        }
        let off_buf = size_of::<BtreeBlock>();
        let size_rec = size_of::<TVal>();
        let mut rec_offset = off_buf;
        let key_size = size_of::<TKey>();
        let mut last_rec: Option<(u16, TKey, TVal)> = Option::None;
        for i in 0..n_recs {
            let rec: TVal = bincode::deserialize(&buf[rec_offset as usize..]).unwrap();
            let rec_key: TKey =
                bincode::deserialize(&buf[rec_offset as usize..rec_offset + key_size]).unwrap();
            if rec_key > *key {
                break;
            }
            // 如果找到了，就返回
            if rec_key == *key {
                return Option::Some((i, rec_key, rec));
            }
            // 如果当前记录的 key 小于搜索 key, 说明搜索 key 所对应的记录要么没有, 要么在之后
            // 所以先把当前记录存起来, 这样万一后面没有, 就先返回当前记录
            if rec_key < *key {
                last_rec = Option::Some((i, rec_key, rec));
            }
            rec_offset += size_rec;
        }
        last_rec
    }

    // pub fn decode_recs<Tkey, TRec>(
    //     &self,
    //     node_hdr: &Option<BtreeBlock>,
    //     buf: &[u8],
    // ) -> Vec<(Tkey, TRec)> {
    //     let mut recs = Vec::new();
    //     if node_hdr.is_none() {
    //         return recs;
    //     }
    //     let node = node_hdr;
    //     let mut offset = 0;
    //     for _ in 0..node.numrecs {}
    //     recs
    // }
}

#[test]
fn test_btree() {
    let dev_size = 10 * 1024; // 10KB
    let bto: BtreeOperator<[u8; 8], [u8; 16]> = BtreeOperator::new(
        Box::new(FileBlockDevice::create("test_btree.bin", dev_size)),
        0,
    );
    bto.create_btree();
    for i in 0..10 {
        let v = (i..i + 16).collect::<Vec<_>>();
        bto.set(
            &v[..8].try_into().expect(""),
            &v[..16].try_into().expect(""),
        );
    }
}
