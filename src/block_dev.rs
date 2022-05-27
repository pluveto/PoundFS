use crate::util::hex_str;

pub trait BlockDevice: Send + Sync {
    fn read_block(&self, block_id: usize, buf: &mut [u8]) -> bool;
    fn write_block(&self, block_id: usize, buf: &[u8]) -> bool;
    fn get_phy_block_size(&self) -> u16;

    /// 读取指定字节位置的数据到缓冲区。禁止跨块写入。
    ///
    /// * `offset` - 字节偏移量
    /// * `buf` - 缓冲区
    fn read_at(&self, offset: usize, buf: &mut [u8]) -> bool {
        // prevent read across blocks
        assert!(buf.len() <= self.get_phy_block_size() as usize);
        // 计算物理块号
        let block_id = offset / self.get_phy_block_size() as usize;
        // 计算物理块内偏移量
        let block_offset = offset % self.get_phy_block_size() as usize;
        // 存放读取的整块数据
        let mut block_buf = vec![0; self.get_phy_block_size() as usize];
        // 读取物理块
        if self.read_block(block_id, &mut block_buf) {
            let block_len = block_buf.len();
            let copy_len = if block_len - block_offset < buf.len() {
                block_len - block_offset
            } else {
                buf.len()
            };
            buf[..copy_len].copy_from_slice(&block_buf[block_offset..(block_offset + copy_len)]);
            true
        } else {
            false
        }
    }
    /// 将缓冲区的数据写入指定字节位置。禁止跨块写入。
    ///
    /// * `offset` - 字节偏移量
    /// * `buf` - 缓冲区
    ///
    fn write_at(&self, offset: usize, buf: &[u8]) -> bool {
        // prevent write across blocks
        assert!(buf.len() <= self.get_phy_block_size() as usize);
        // 计算物理块号
        let block_id = offset / self.get_phy_block_size() as usize;
        // 计算物理块内偏移量
        let block_offset = offset % self.get_phy_block_size() as usize;

        assert!(block_offset + buf.len() <= self.get_phy_block_size() as usize);

        // 存放原先的整块数据
        let mut block_buf = vec![0; self.get_phy_block_size() as usize];
        // 读取物理块
        if self.read_block(block_id, &mut block_buf) {
            // 物理块长度
            let block_len = block_buf.len();
            // 复制长度（因为 0 到 block_offset 的数据不能被覆盖，所以应当只复制后部分到 block_buf）
            let copy_len = if block_len - block_offset < buf.len() {
                block_len - block_offset
            } else {
                buf.len()
            };
            block_buf[block_offset..(block_offset + copy_len)].copy_from_slice(&buf[..copy_len]);
            self.write_block(block_id, &block_buf)
        } else {
            false
        }
    }
    /// 跨块连续读取
    fn read_all_at(&self, offset: usize, buf: &mut [u8]) -> bool {
        let mut offset = offset;
        let mut remain_len = buf.len();
        let mut buf_offset = 0;
        let phy_block_size = self.get_phy_block_size() as usize;
        let first_block_offset = offset % phy_block_size;
        /*
         * 
         *          <-block_off-->
         * buf                    |xxxxx|xxxxxxxxxxxxxxxxx|
         * 
         * device:  |     phy block     |     phy block     |
         * 
         */
        // if we have block_off, we need to skip it first, copy range is block_off end to block end
        if first_block_offset > 0 {
            let mut block_buf = vec![0; phy_block_size];
            if !self.read_block(offset / phy_block_size, &mut block_buf) {
                return false;
            }
            let copy_len = if first_block_offset + remain_len > phy_block_size {
                phy_block_size - first_block_offset
            } else {
                remain_len
            };
            buf[buf_offset..(buf_offset + copy_len)].copy_from_slice(&block_buf[first_block_offset..(first_block_offset + copy_len)]);
            buf_offset += copy_len;
            remain_len -= copy_len;
            offset += copy_len;
        }
        // copy remain_len bytes. and now its aligned so we just copy
        while remain_len > 0 {
            let mut block_buf = vec![0; phy_block_size];
            if !self.read_block(offset / phy_block_size, &mut block_buf) {
                return false;
            }
            let copy_len = if remain_len > phy_block_size {
                phy_block_size
            } else {
                remain_len
            };
            buf[buf_offset..(buf_offset + copy_len)].copy_from_slice(&block_buf[..copy_len]);
            buf_offset += copy_len;
            remain_len -= copy_len;
            offset += copy_len;
        }        
        true
    }
    /// 跨块连续写入
    fn write_all_at(&self, offset: usize, buf: &[u8]) -> bool {
        // 当前字节偏移
        let mut offset = offset;
        // 缓冲剩余未写入长度
        let mut remain_buf_len = buf.len();
        // 缓冲内偏移
        let mut buf_offset = 0;
        let first_block_offset = offset % self.get_phy_block_size() as usize;
        // 如果第一个块偏移不为0，则需要先写入第一个块，这样后面的写入才能对齐
        if first_block_offset > 0 {
            let pblk_id = offset / self.get_phy_block_size() as usize;
            let mut block_buf = vec![0; self.get_phy_block_size() as usize];
            if !self.read_block(pblk_id, &mut block_buf) {
                return false;
            }
            let write_len = if first_block_offset + remain_buf_len > self.get_phy_block_size() as usize {
                self.get_phy_block_size() as usize - first_block_offset
            } else {
                remain_buf_len
            };
            block_buf[first_block_offset..(first_block_offset + write_len)].copy_from_slice(&buf[buf_offset..(buf_offset + write_len)]);
            self.write_block(pblk_id, &block_buf);
            offset += write_len;
            buf_offset += write_len;
            remain_buf_len -= write_len;
        }
        // 写入剩余数据
        while remain_buf_len > 0 {
            let mut block_buf = vec![0; self.get_phy_block_size() as usize];
            let pblk_id = offset / self.get_phy_block_size() as usize;
            // 当写入的长度不满一个物理块时，需要先从磁盘读取数据
            if remain_buf_len < self.get_phy_block_size() as usize {
                if !self.read_block(pblk_id, &mut block_buf) {
                    return false;
                }
            }
            let write_len = if remain_buf_len > self.get_phy_block_size() as usize {
                self.get_phy_block_size() as usize
            } else {
                remain_buf_len
            };
            block_buf[..write_len].copy_from_slice(&buf[buf_offset..(buf_offset + write_len)]);
            self.write_block(pblk_id, &block_buf);
            offset += write_len;
            buf_offset += write_len;
            remain_buf_len -= write_len;
        }
        true
    }
}
