use std::mem::size_of;

use serde::{Deserialize, Serializer, Serialize};

use crate::{
    block_dev::BlockDevice,
    btree::{AllocRec, BtreeBlock},
    file_blk::FileBlockDevice,
};

struct BtreeOperator<TKey, TVal> {
    dev: Box<dyn BlockDevice>,
    btroot_block: u64,
    __type_key: std::marker::PhantomData<TKey>,
    __type_val: std::marker::PhantomData<TVal>,
}
impl<'de, TKey, TVal> BtreeOperator<TKey, TVal>
where
    TKey: Deserialize<'de> + PartialOrd,
    TVal: Deserialize<'de> + Serialize,
{
    fn new(dev: Box<dyn BlockDevice>, btroot_block: u64) -> Self {
        BtreeOperator {
            dev,
            btroot_block,
            __type_key: std::marker::PhantomData,
            __type_val: std::marker::PhantomData,
        }
    }

    pub fn get<'a>(&self, key: &TKey) -> Option<TVal> {
        let mut buf = [0u8; 512];
        // 读一个 block
        self.dev.read_block(self.btroot_block as usize, &mut buf);
        // -- node 为 block 的头部
        let node: BtreeBlock = bincode::deserialize(&buf).unwrap();
        // -- recs 为 block 的剩余部分
        self.find_rec_in_block(&node, &buf, key)        
    }

    pub fn set(&self, key: &TKey, value: &TVal) -> bool {
        // let mut buf = [0u8; 512];
        // // 读一个 block
        // self.dev.read_block(self.btroot_block as usize, &mut buf);
        // // -- node 为 block 的头部
        // let header: BtreeBlock = bincode::deserialize(&buf).unwrap();
        // // -- recs 为 block 的剩余部分
        // let ret: std::option::Option<TVal> = self.find_rec_in_block(&header, &buf, key);
        // // 未找到
        // if ret.is_none() {
        //     // 写回
        //     let off_new_rec_opt = self.get_off_new_rec(&header, &buf, key);
        //     if off_new_rec_opt.is_none() {
        //         // TODO: 更复杂的 b+tree 操作
        //         return true;
        //     }
        //     let off_new_rec = off_new_rec_opt.unwrap();
        //     let rec_buf = bincode::serialize(value).unwrap();
        //     // update buf with new rec
        //     // buf[off_new_rec as usize..512].copy_from_slice(&rec_buf);
        //     // buf[0] = 1;

        //     return true;
        // } else {
        //     return true;
        // }
        return true;
    }
    /// 获取最后一个记录的末端+1, 即追加记录的位置. 若返回 None, 则代表满, 无法追加记录
    pub fn get_off_new_rec(
        &self,
        header: &BtreeBlock,
        buf: &'de [u8],
        key: &TKey,
    ) -> Option<usize> {
        let n_recs = header.numrecs;
        let sz_recs = size_of::<TVal>();
        let sz_header = size_of::<BtreeBlock>();
        let sz_block = buf.len();
        let off_new_rec = sz_header + sz_recs * (n_recs + 1) as usize;
        if off_new_rec > sz_block {
            return None;
        }
        return Some(off_new_rec);
    }
    /// 读取与 key 匹配的记录, 或者在 key 之前最近的记录. 如果未找到, 则返回 None
    pub fn find_rec_in_block(
        &self,
        header: &BtreeBlock,
        buf: &'de [u8],
        key: &TKey,
    ) -> Option<TVal> {
        let n_recs = header.numrecs;
        let off_buf = size_of::<BtreeBlock>();
        let size_rec = size_of::<TVal>();
        let mut rec_offset = off_buf;
        let key_size = size_of::<TKey>();
        let mut last_rec: Option<TVal> = Option::None;
        for _i in 0..n_recs {
            let rec: TVal = bincode::deserialize(&buf[rec_offset as usize..]).unwrap();
            let rec_key: TKey = bincode::deserialize(&buf[rec_offset as usize..key_size]).unwrap();
            // 如果找到了，就返回
            if &rec_key == key {
                return Option::Some(rec);
            }
            // 如果当前记录的 key 小于搜索 key, 说明搜索 key 所对应的记录要么没有, 要么在之后
            // 所以先把当前记录存起来, 这样万一后面没有, 就先返回当前记录
            if rec_key < *key {
                last_rec = Option::Some(rec);
            }
            if rec_key > *key {
                break;
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
    let devSize = 10 * 1024; // 10KB
    let bto:BtreeOperator<&str, &str> = BtreeOperator::new(
        Box::new(FileBlockDevice::create("test_btree.bin", devSize)),
        0,
    );
    for i in 0..10 {
        let mut key_str = String::new();
        key_str.push_str("key_");
        key_str.push_str(&i.to_string());
        let key = key_str.as_str();
        let mut val_str = String::new();
        val_str.push_str("val_");
        val_str.push_str(&i.to_string());
        let val = val_str.as_str();
        
        bto.set(&key, &val);
    }
}
