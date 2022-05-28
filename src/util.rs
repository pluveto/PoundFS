use rand::RngCore;

pub fn pad_zeroes<const A: usize, const B: usize>(arr: [u8; A]) -> [u8; B] {
    assert!(B >= A); //just for a nicer error message, adding #[track_caller] to the function may also be desirable
    let mut b = [0; B];
    b[..A].copy_from_slice(&arr);
    b
}

pub fn hex_str(number: usize) -> String {    
    // format to hex, space per 8 char
    format!("{:08x}H", number)
    
}

pub fn human_readable_size(size: usize) -> String {
    let mut size = size;
    let mut unit = "B";
    if size > 1024 {
        size /= 1024;
        unit = "KB";
    }
    if size > 1024 {
        size /= 1024;
        unit = "MB";
    }
    if size > 1024 {
        size /= 1024;
        unit = "GB";
    }
    if size > 1024 {
        size /= 1024;
        unit = "TB";
    }
    if size > 1024 {
        size /= 1024;
        unit = "PB";
    }
    if size > 1024 {
        size /= 1024;
        unit = "EB";
    }
    if size > 1024 {
        size /= 1024;
        unit = "ZB";
    }
    if size > 1024 {
        size /= 1024;
        unit = "YB";
    }
    format!("{} {}", size, unit)
}

pub fn ffs(number: u32) -> u8 {
    let mut number = number;
    let mut i = 0;
    while number != 0 {
        number >>= 1;
        i += 1;
    }
    i
}

pub fn uuid() -> [u8; 16] {
    let mut rng = rand::thread_rng();
    let mut uuid = [0; 16];
    rng.fill_bytes(&mut uuid);
    uuid
}