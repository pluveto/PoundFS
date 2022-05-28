use serde::{Deserialize, Serialize};

use crate::{dstruct::UUID, util::uuid};
const BtreeBlockMagicNum: u32 = 0xB5B1A1A1;
// https://stackoverflow.com/questions/32428153/how-can-i-align-a-struct-to-a-specified-byte-boundary
#[repr(align(64))]
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct BtreeBlock {
    /* 8 bytes */
    pub magicnum: u32,    // B+树块的 Magic Number AB3B
    pub level: u16, // B+树块的层级，如果level是0，表示当前的 block 存储的都是叶子节点，否则是索引节点。越靠近根部越大
    pub numrecs: u16, // 如果是叶子节点，表示当前 B+树块中的记录数。否则表示有多少子节点（AllocRec）。
    /* 16 bytes */
    /* long form block header */
    pub leftSibling: u64,  // 左兄弟节点的块号
    pub rightSibling: u64, // 右兄弟节点的块号
    /* 32 bytes */
    pub blkno: u64, // 当前节点的块号。字节数/512 可得之
    pub lsn: u64,   // 最后写入节点的日志 SN（序列号）
    pub uuid: UUID, // 当前节点的UUID
    /* 8 bytes */
    pub owner: u32, // 当前节点的所有者（即哪个 ag）
    pub crc: u32,   // 本块的 CRC 校验值
}

impl BtreeBlock {
    pub fn new(blkno: u64) -> Self {
        BtreeBlock {
            magicnum: BtreeBlockMagicNum,
            level: 0,
            numrecs: 0,
            leftSibling: 0,
            rightSibling: 0,
            blkno: blkno,
            lsn: 0,
            uuid: uuid(),
            owner: 0,
            crc: 0,
        }
    }
}

// AllocRec 是一个数对。
// 用来表示空闲空间时，AllocRec 表示每个空闲块的起始块号和长度。
#[repr(align(8))]
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct AllocRec {
    pub startblock: u32, // 当前记录的起始块号
    pub blockcount: u32, // 当前记录的块数
}
