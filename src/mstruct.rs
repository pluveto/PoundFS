pub type AbsInoNo = u64;
pub type UUID = [u8; 16];
use serde::{Serialize, Deserialize};

const SuperBlockMagicNumber: u32 = 0x73666470;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SuperBlock {
    pub magicnum: u32,      // 魔数
    pub blocksize: u32,     // 逻辑块大小 通常是4096字节（4KB）
    pub dblocks: u32,       // 数据块数
    pub rblocks: u32,       // 实时块数 https://www.cnblogs.com/orange-CC/p/12711078.html
    pub rextents: u32,      // 实时扩展数
    pub uuid: UUID,         // 唯一标识符
    pub rootino: u32,       // 根目录Inode号
    pub rbmino: u32,        // 实时块位图Inode号
    pub rextsize: u32,      // 实时扩展块大小
    pub agblocks: u32,      // 块组大小 最后一个AG的实际大小可能会不一样
    pub agcount: u32,       // 块组数
    pub rbmblocks: u32,     // 实时块位图块数
    pub logblocks: u32,     // 日志块数
    version: u16,           // 版本号
    pub sectsize: u16,      // 扇区大小 bytes
    pub inodesize: u16,     // Inode大小 bytes
    pub inopblock: u16,     // Inode per block
    pub fsname: [u8; 16],   // 文件系统名称
    pub blocksize_bits: u8, // 块大小位数，即以2为底的对数
    pub sectsize_bits: u8,  // 扇区大小位数，即以2为底的对数
    pub inodesize_bits: u8, // Inode大小位数，即以2为底的对数
    pub inpblock_bits: u8,  // Inode per block位数，即以2为底的对数
    pub agblocks_bits: u8,  // 块组大小位数，即以2为底的对数
    pub rextents_bits: u8,  // 实时扩展块大小位数，即以2为底的对数
    pub inprogress: u8,     // 是否正在进行格式化
    pub imax_pct: u8,       // 最大Inode百分比
    pub icount: u64,        // Inode总数
    pub ifree: u64,         // 空闲Inode总数
    pub fdblocks: u64,      // 空闲数据块总数
    pub frextents: u64,     // 空闲实时扩展总数

    pub uquotino: AbsInoNo, // 用户配额Inode号
    pub gquotino: AbsInoNo, // 组配额Inode号
    pub qflags: u32,        // 配额标志
    pub flags: u32,         // 混合标志
    pub sb_inoalignmt: u32, // Inode对齐值 Inode chunk alignment, fsblocks
}

impl SuperBlock {
    pub fn new() -> Self {
        SuperBlock {
            magicnum: SuperBlockMagicNumber,
            blocksize: 0,
            dblocks: 0,
            rblocks: 0,
            rextents: 0,
            uuid: [0; 16],
            rootino: 0,
            rbmino: 0,
            rextsize: 0,
            agblocks: 0,
            agcount: 0,
            rbmblocks: 0,
            logblocks: 0,
            version: 0,
            sectsize: 0,
            inodesize: 0,
            inopblock: 0,
            fsname: [0; 16],
            blocksize_bits: 0,
            sectsize_bits: 0,
            inodesize_bits: 0,
            inpblock_bits: 0,
            agblocks_bits: 0,
            rextents_bits: 0,
            inprogress: 0,
            imax_pct: 0,
            icount: 0,
            ifree: 0,
            fdblocks: 0,
            frextents: 0,
            uquotino: 0,
            gquotino: 0,
            qflags: 0,
            flags: 0,
            sb_inoalignmt: 0,
        }
    }
}
