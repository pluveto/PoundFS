#[cfg(test)]
use crate::util::pad_zeroes;
#[cfg(test)]
use crate::{block_dev::BlockDevice, file_blk::FileBlockDevice};

#[test]
fn test_file_blk() {
    let path = "test_file_blk.bin";
    let size = 1024 * 1024 * 50; // 50MB
    let file_blk = FileBlockDevice::create(path, size);
    let mut buf = [0u8; 512];
    let buf_to_write: [u8; 512] = pad_zeroes([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    file_blk.write_block(0, &buf_to_write);
    file_blk.read_block(0, &mut buf);
    let mut long_buf_to_write: [u8; 5120] = pad_zeroes([]);
    for i in 0..512 {
        long_buf_to_write[i * 8] = 'A' as u8 + i as u8 % 26;
    }
    for i in 8 * 512..5120 {
        long_buf_to_write[i] = '@' as u8;
    }
    long_buf_to_write[5120 - 1] = '#' as u8;
    file_blk.write_all_at(0x00000100, long_buf_to_write.as_mut());
    assert_eq!(buf, buf_to_write);
}
