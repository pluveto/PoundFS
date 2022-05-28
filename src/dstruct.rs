pub type AbsInoNo = u64;
pub type UUID = [u8; 16];


use serde::{Serialize, Deserialize};

use crate::util::uuid;

const SuperBlockMagicNum: u32 = 0x73666470;
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
            magicnum: SuperBlockMagicNum,
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
// AG第二个扇区包括两个空闲空间的B+树和AG空闲空间
// XFS_BTNUM_AGF Btree number 0 is bno, 1 is cnt.  This value gives the size of the arrays below.
const AgfBtNum: usize = 3;
const AgfMagicNum: u32 = 0x01231023;
pub struct Agf {
    magicnum: u32,   // AG扇区的 Magic Number
    versionnum: u32, // 版本号
    seqno: u32,      // 扇区的 AG 序号，from 0
    length: u32,     // AG 中有多少块，除了最后一块外都相同，等于 agblocks
    /**
     * 从下标 0 开始，依次是
     * bnoroot  表示“以块号为索引的 FS B+tree 根节点”的块号
     * cntroot  表示“以块数为索引的 FS B+tree 根节点”的块号
     * rmaproot 表示“表示对已用空间的倒排索引 B+tree 根节点”的块号
     */
    pub roots: [u32; AgfBtNum], // 用来管理空间所需的几个结构的头地址
    pub spare0: u32,     // 有意留空的字段
    pub levels: [u32; AgfBtNum], // B+树的层级或深度，与agf_roots一一对应。对于新创建的AG，三个 level 都是 1
    pub spare1: u32,                  // 占位

    pub flfirst: u32, // FL(空闲链表)块的开始位置
    pub fllast: u32,  // FL 块的结束位置。
    pub flcount: u32, // FL 中的块数量。

    pub freeblks: u32,  // AG 空闲块数量
    pub longest: u32,   // 最长连续空闲块长度
    pub btreeblks: u32, // AGF 的 B+Tree 使用掉的块数
    pub uuid: UUID,     // 当前 AGF 的UUID
    pub spare64: [u64; 16],
    pub lsn: u64, // 最后写入 AGF 的日志 SN（序列号）
    pub crc: u32, // AGF 的 CRC 校验值

    pub spare2: u32, // 占位
}

impl Agf {
    // xfs_agfblock_init
    // * `agno` - AG number from 0
    // * `agblocks` - AG real size in blocks
    pub fn new(agno: u32, agblocks: u32) -> Self {
        Agf {
            magicnum: AgfMagicNum,
            versionnum: 0,
            seqno: agno,
            length: agblocks,
            roots: [0; AgfBtNum],
            spare0: 0,
            levels: [0; AgfBtNum],
            spare1: 0,
            flfirst: 1,
            fllast: 0,
            flcount: 0,
            freeblks: 0,
            longest: 0,
            btreeblks: 0,
            uuid: uuid(),
            spare64: [0; 16],
            lsn: 0,
            crc: 0,
            spare2: 0,
        }
    }
}

pub struct Agfl {
    magicnum: u32, // AGFL 的 Magic Number
    seqno: u32,    // AGFL 的隶属 AG 序号，from 0
    uuid: UUID,    // AGFL 的UUID
    lsn: u64,      // 最后写入 AGFL 的日志 SN（序列号）
    crc: u32,      // AGFL 的 CRC 校验值
                   // 剩余的整个扇区空间都是 AGFL 的有效部分
}


pub struct Agi {
    magicnum: u32,   // AGInode 的 Magic Number = XAGI
    versionnum: u32, // AGInode 的版本号
    seqno: u32,      // 所属 AG 的序号，from 0
    length: u32,     // 当前 AG 的大小，单位是块

    count: u32,     // 当前 AG 已分配的 inode 数量
    root: u32,      // inobt 根节点的位置（块号）
    level: u32,     // inobt 的深度
    freecount: u32, // 已分配但尚未使用的 inode 数量

    newino: u32,         // 最新分配的 inode
    dirino: u32,         // 空余字段
    unlinked: [u32; 64], //哈希表，记录已经 unlink 但仍然被引用的 inode。默认为全 -1，代表无占用。
    uuid: UUID,          // 当前 文件系统的 UUID
    crc: u32,            // AGI 扇区的 CRC 校验值
    pad32: u32,          // 填充对齐
    lsn: u64,            // 最后写入 AGI 的日志 SN（序列号）

    freeRoot: u32,  // finobt （空闲 inode 树）的根节点
    freeLevel: u32, // finobt 的层级

    iblocks: u32, // inobt 已用块数。需要启用 inobt count 特性
    fblocks: u32, // finobt 已用块数。需要启用 inobt count 特性
}

pub struct InodeBtreeRecord {
    startino: u32, // 一个 inode chunk 里 inode num 最小的那 inode，也就是这个 chunk 的起始 inode
    holemask: u16, // 空洞掩码。稀疏 inode 允许以小于 chunk 的大小分配 inodes，从而 chunk 中有的位置需要跳过。
    // 16 位，每位代表 4 个连续 inode 空洞。
    count: u8, // 表示一共有多少已分配的 inode，当未启用 sparse 时为 64，但若启用了 sparse，则为 64 - 4 * n(holemask)
    freecount: u8, // 当前记录的空余 inode 数量（已分配但尚未使用的 inode 数量）
    free: u64, // finode 位图，64 位对应 inode chunk 里的 inode 的空闲情况。1 表示可用。
}

// unix 纳秒时间戳
struct timestamp {
    sec: u32,
    nsec: u32,
}

type InodeFlags = u16;

enum InodeFlag {
    XfsDiflagRealtime = 1 << 0, // 表示当前 inode 的数据位于在 realtime 设备上。
    XfsDiflagPrealloc = 1 << 1, // 表示当前 inode 含有预分配的 extent。
    XfsDiflagNewrtbm = 1 << 2,  // 表示当前 inode 使用 real-time bitmap格式，参照sb_rbmino。
    XfsDiflagImmutable = 1 << 3, // 用于阻止对当前文件的所有修改，甚至以写权限打开都不可以。
    XfsDiflagAppend = 1 << 4,   // 表示此文件只能追加内容，不能删除。
    XfsDiflagSync = 1 << 5,     // 表示当前当前 inode 的写操作都是同步写操作。
    XfsDiflagNoatime = 1 << 6,  // 表示不更新当前 inode 的atime。
    XfsDiflagNodump = 1 << 7,   // 表示不支持dump操作。
    XfsDiflagRtinherit = 1 << 8, // 用于目录的 inode ，表示其下的 inode 将自动继承 XFS_DIFLAG_REALTIME_BIT
    XfsDiflagProjinherit = 1 << 9, // 也用于目录，表示其下的 inode 将自动继承di_projid*。
    XfsDiflagNosymlinks = 1 << 10, // 也用于目录，表示其下不允许创建符号链接。
    XfsDiflagExtsize = 1 << 11, // 用于文件，指明 real-time 文件的 extent size 或普通文件的暗示性 extent size，参照di_extsize。
    XfsDiflagExtszinherit = 1 << 12, // 用于目录，其下的 inode 继承 di_extsize。
    XfsDiflagNodefrag = 1 << 13, // 表示碎片化整理的时候忽略这个 inode 。
    XfsDiflagFilestream = 1 << 14, // 具有这个标记的目录会提前 reserve AG 的可用空间，并把这部分reserve的空间留给自己下面的inode使用，其它不在此目录下的inode在申请空间时则不会获得这部分空间。
}

enum InodeFlag2 {
    XFS_DIFLAG2_DAX = 1 << 0, // 是和DAX有关的flag，表明当前inode在支持DAX的persistent-memory设备上，并且以DAX的方式访问。如果给目录设置此flag，则表示其下的inode自动继承这个标志。
    XFS_DIFLAG2_REFLINK = 1 << 1, // 表示当前inode有和其它inode共享的数据块，这和reflink特性有关。
    XFS_DIFLAG2_COWEXTSIZE = 1 << 2, // 和XFS_DIFLAG_EXTSZINHERIT类似，只是用于COW(copy-on-write)操作时的“暗示性”extent size。与di_cowextsize配合使用。如果一个目录具有这个标记，则其下所有inode自动继承。
    XFS_DIFLAG2_BIGTIME = 1 << 3, // 这个标记是目前新加的，是XFS为了解决“2038问题”而新增的特性，增加了XFS支持的时间戳长度。具有这个标记的inode表示使用这个特性。
}

pub struct Dinode {
    magic: u16,        // IN
    mode: u16,         // rwx 等权限位。
    version: u8,       // 版本，3
    format: u8,        // 格式，常见有 FMT_LOCAL, FMT_EXTENTS, FMT_BTREE. FMT_DEV 用于字符或块设备
    onlink: u16,       // 已过期
    uid: u32,          // 文件所有人
    gid: u32,          // 文件所属组
    nlink: u32,        // 硬链接计数
    projid_lo: u16,    // project quote id，暂时用不到
    projid_hi: u16,    // project quote id，暂时用不到
    pad: [u8; 6],      // 占位，用不到
    flushiter: u16,    //
    atime: timestamp,  // 最后访问时间
    mtime: timestamp,  // 最后修改时间
    ctime: timestamp,  // 最后 inode 状态修改时间
    size: u64, // Inode的大小，对于文件inode来说并不是其实际占用空间的大小，而是看EOF的位置。对于目录inode来说就是目录条目所占的空间。
    nblocks: u64, // 统计此 inode 占用的文件系统块数。
    extsize: u32, // 用于实时设备，暂时用不到
    nextents: u32, // 暂时用不到
    anextents: u16, // 暂时用不到
    forkoff: u8, // datafork 和 attrfork 的分界线。乘以8等到真正的偏移字节量
    aformat: i8, // 指明此inode组织扩展属性数据时使用的数据结构，1 表示 LOCAL
    dmevmask: u32, // 过期，无意义
    dmstate: u16, // 过期，无意义
    flags: InodeFlags, // inode 标记
    gen: u32,  // 一个随机数，每个 inode 不同

    /* di_next_unlinked is the only non-core field in the old dinode */
    next_unlinked: u32, // 之前提到 unlinked 哈希表，记录已经 unlink 但仍然被引用的 inode。此处是该哈希表的拉链。

    /* start of the extended dinode, writable fields */
    crc: u32,         // 当前inode的内容的CRC校验值
    changecount: u64, // inode 的 i_version，每次修改 inode 时加 1
    lsn: u64,         // 最后写入操作的 Log SN
    flags2: u64,      //     扩展的 flags，上面的 flags 不够用了
    cowextsize: u32,  //
    pad2: [u8; 12],   // 占位

    /* fields only written to during inode creation */
    crtime: timestamp, // 创建时间
    ino: u64,          // 绝对 inode number
    uuid: UUID,
    /* structure must be padded to 64 bit alignment */
}

enum ExtentState {
    ExtNorm,         // 正常状态。有数据写入状态
    ExtUnwritten,    // 表示当前extent处于预分配但是还没有实际数据写入的状态
    ExtDmapiOffline, // 暂时用不到
    ExtInvalid,      // 暂时用不到
}

pub struct BmbtRecord {
    startoff: u64,      // 文件的逻辑偏移块号，属于文件size内的逻辑偏移
    startblock: u32,    // 此extent相对于整个文件系统的起始物理块号。
    blockcount: u64,    // 此extent包含多少个块。
    state: ExtentState, // 此extent的一个标记位
}

pub struct BmdrBlock {
    level: u8, // 深度，0 表示叶节点
    numrecs: u8, // 当前block里有多少records
               // 后面是 records，键值对
               // 如果是叶节点：key 是逻辑块号，value 是物理块号
               // 如果是中间节点：key 是逻辑块起始地址，value 是子节点的指针
}

// xfs_dir2_sf_hdr
pub struct DirShortFormatHeader {
    count: u16,      // 此目录 项的个数
    i8count: u8,     // 表示有多少目录的条目是用于64位inode的。
    parent: [u8; 8], // 父目录的inode号
}

pub struct DirShortFormatEntry {
    namelen: u8,     // 文件名长度
    offset: [u8; 2], // 偏移，用于辅助在readdir的时候迭代目录内容用的。
    name: [u8],      // 文件名，flexible array
}

pub struct DirDataHeader {
    hdr: DirBlockHeader,
    best_free: [DirDataFree; 3],
    pad: u32, /* 64 bit alignment */
}

pub struct DirBlockHeader {
    magic: u32,
    crc: u32,
    blkno: u64,
    lsn: u64,
    uuid: UUID,
    owner: u64,
}

pub struct DirDataFree {
    offset: u16, /* start of freespace */
    length: u16, /* length of freespace */
}

pub struct DirDataEntry {
    inumber: u64,
    namelen: u8,
    name: [u8],
    // filetype: u8,
    // tag: u16,
}
pub struct DirDataUnused {
    freetag: u16, // freetag是一个magic number表示此entry是unused的
    length: u16,  // length表示当前这个unused空间的长度。

    tag: u16, // tag就是offset，表示当前这个entry相对于其所在directory block的偏移地址
}

pub struct DirLeafEntry {
    hashval: u32, // 根据name计算出来的一个hash值
    address: u32, // hash所对应的entry在此directory block内的偏移地址，单位是 8 字节。乘以 8 得 实际地址
}

pub struct BlockTail {
    count: u32, // 记录当前directory block有多少leaf
    stale: u32, // 表示count记录的总数中有多少free的leaf
}

pub struct DirLeaf {
    hdr: DirLeafHeader,
    __ents: [DirLeafEntry],
}
pub struct DirLeafHeader {
    info: DirAttrBlockInfo,
    count: u16, //当前leaf block中有多少个entry
    stale: u16, // 当前leaf block count 中有多少个 unused entry
    pad: u32,
}

pub struct DirLeaftTail {
    bestcount: u32, // 有多少 directory block 用于存储数据
}

pub struct DirAttrBlockInfo {
    // hdr
    forw: u32, // 链表指针
    back: u32, // 链表指针
    magic: u16,
    pad: u16,

    crc: u32,
    blkno: u64,
    lsn: u64,
    uuid: UUID,
    owner: u64,
}

// free index block

struct DirFreeHeader {
    hdr: DirBlockHeader,
    firstdb: u32, // 表示当前best free space(简称bests)数组是从哪个directory block号开始记录的
    nvalid: u32,  // 当前目录的di_size范围内逻辑上能放下多少directory block
    nused: u32,   // 当前有多少在用的directory block
    pad: u32,     /* 64 bit alignment */
}

struct DirFree {
    hdr: DirFreeHeader,
    bests: [u16], /* best free counts */
                  /* unused entries are -1 */
}

// node block
// xfs_da3_node_hdr 
struct DirAttrNodeHeader {
    info: DirAttrBlockInfo,
    __count: u16, /* count of active entries */
    __level: u16, /* level above leaves (leaf == 0) */
    __pad32: u32,
}
// xfs_da_node_entry 
struct DirAttrNodeEntry {
    hashval: u32, /* hash value for this descendant */
    before: u32,  /* Btree block before this key */
}
// xfs_da3_intnode 
struct DirAttrInodeTreeNode {
    hdr: DirAttrNodeHeader,
    __btree: [DirAttrNodeEntry],
}
