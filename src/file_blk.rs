use std::{
    fs::File,
    io::{Read, Seek, SeekFrom, Write},
};

use crate::block_dev::BlockDevice;

pub struct FileBlockDevice {
    file: File,
}

impl FileBlockDevice {
    pub fn new(path: &str) -> Self {
        FileBlockDevice {
            file: File::options().read(true).write(true).open(path).unwrap(),
        }
    }
    pub fn create(path: &str, size: usize) -> Self {
        let output = File::create(path);
        if output.is_err() {
            panic!("Failed to create file {}", path);
        }
        let file = output.unwrap();
        file.set_len(size as u64).unwrap();
        drop(file);
        FileBlockDevice::new(path)
    }

    // drop
    pub fn drop(&mut self) {
        self.file.sync_all().unwrap();
    }
}

const PHY_BLOCKSIZE: u16 = 512;

impl BlockDevice for FileBlockDevice {
    fn read_block(self: &FileBlockDevice, block_id: usize, buf: &mut [u8]) -> bool {
        let mut file = self.file.try_clone().unwrap();
        file.seek(SeekFrom::Start((block_id * PHY_BLOCKSIZE as usize) as u64))
            .unwrap();
        file.read_exact(buf).unwrap();
        return true;
    }

    fn write_block(self: &FileBlockDevice, block_id: usize, buf: &[u8]) -> bool {
        let mut file = self.file.try_clone().unwrap();
        file.seek(std::io::SeekFrom::Start((block_id * PHY_BLOCKSIZE as usize) as u64))
            .unwrap();
        file.write_all(buf).unwrap();
        return true;
    }

    fn get_phy_block_size(self: &FileBlockDevice) -> u16 {
        PHY_BLOCKSIZE 
    }
}
