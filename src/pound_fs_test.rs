#[cfg(test)]
use crate::file_blk::FileBlockDevice;
#[cfg(test)]
use crate::pound_fs::MkfsOption;
#[cfg(test)]
use crate::pound_fs::make_fs;
#[test]
fn test_make_fs(){
    let fsize =  1024 * 1024 * 50; // 50MB
    let dev = FileBlockDevice::create("test_make_fs.bin", fsize);
    let mkfs_ptions = MkfsOption{
        size: fsize,
        agblocks: 10240,
        blocksize: 4096,
    };
    make_fs(Box::new(dev), mkfs_ptions);

}