use crate::{block_dev::BlockDevice, dstruct::{SuperBlock, Agf}, util::{human_readable_size, hex_str, ffs}, mstruct::{AgCtx, AgfCtx}};

pub struct MkfsOption {
    pub size: usize,    // 总大小，单位为字节
    pub blocksize: u32, // 逻辑块大小 通常是4096字节（4KB）
    pub agblocks: u32,  // 每个 AG 的逻辑块数
}

// xfs_mount
pub struct MountPoint<'a> {
    pub dev: Box<dyn BlockDevice + 'a>,
    superblock: SuperBlock,
}

impl<'a> MountPoint<'a> {
    pub fn new(dev: Box<dyn BlockDevice + 'a>, superblock: SuperBlock) -> Self {
        MountPoint { dev, superblock }
    }
}
/// 基于块设备创建文件系统（格式化）
/// refs: xfs_readsb
pub fn make_fs(dev: Box<dyn BlockDevice>, opt: MkfsOption) {
    // init MountPoint
    let mut mp =MountPoint::new(
        dev,
        SuperBlock::new(),
    );
    mp.superblock.blocksize = opt.blocksize;
    mp.superblock.blocksize_bits = ffs(opt.blocksize) - 1;
    mp.superblock.agblocks = opt.agblocks;
    mp.superblock.agblocks_bits = ffs(opt.agblocks) - 1;
    mp.superblock.dblocks = (opt.size / opt.blocksize as usize) as u32;
    // sector_size = xfs_getsize_buftarg(mp->m_ddev_targp);
    // #define xfs_getsize_buftarg(buftarg)	block_size((buftarg)->bt_bdev)
    // 因此推断 blocksize 就是扇区大小

    // 将设备划分为多个 AG

    // - 每个 AG 的字节大小
    let ag_size = opt.blocksize * opt.agblocks;
    // - 总 AG 数
    let ag_count = (opt.size as f64 / ag_size as f64).ceil() as usize;
    // - 最后一个 AG 的实际大小
    let last_ag_size = (opt.size as u32) % (ag_size as u32);

    mp.superblock.agcount = ag_count as u32;
    println!(
        "size={}, (each)ag_size={}, (total)ag_count={}, last_ag_size={}\n",
        human_readable_size(opt.size),
        human_readable_size(ag_size as usize),
        ag_count,
        human_readable_size(last_ag_size as usize)
    );

    for ag_no in 0..(ag_count) {
        let cur_ag_size = if (ag_no + 1) == ag_count {
            last_ag_size
        } else {
            ag_size
        };
        init_ag(
            &mp,
            &InitAgOption {
                ag_size: cur_ag_size,
                start_block: ag_no * opt.agblocks as usize,
                ag_no: ag_no as u32,
            },
        );
    }
}

pub struct InitAgOption {
    pub ag_no: u32,
    pub ag_size: u32,
    pub start_block: usize, // 起始物理块
}
// xfs_ag_init_headers
pub fn init_ag(mp: &MountPoint, opt: &InitAgOption) {
    println!(
        "init_ag: ag_no={}, ag_size={}, start_block={}",
        opt.ag_no,        
        human_readable_size(opt.ag_size as usize),
        opt.start_block
    );    
    println!("will init ag at block {}", opt.start_block);
    

    // SB - sec 0
    let sb_sector_off = opt.start_block * mp.superblock.blocksize as usize;
    println!("write superblock to addr {}", hex_str(sb_sector_off));
    let sb_encoded = bincode::serialize(&mp.superblock).unwrap();
    mp.dev.as_ref().write_all_at(sb_sector_off, sb_encoded.as_slice());
    
    // AGF - sec 1
    let agf_sector_off = (opt.start_block + 1) * mp.superblock.blocksize as usize;
    println!("write agf to addr {}", hex_str(agf_sector_off));
    // let mut agf = Agf::new();
    // let agfCtx = AgfCtx::new(mp, &mut agf);



}
