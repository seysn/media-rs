#[derive(Debug)]
pub enum Pixel {
    Gray(u8),
    RGB(u8, u8, u8),
}

pub fn make_buffer(buffer: &[u8], bits_per_pixel: usize) -> Vec<Pixel> {
    match bits_per_pixel {
        8 => make_gray_buffer(buffer),
        24 => make_rgb_buffer(buffer),
        _ => unimplemented!("{} bits per pixel", bits_per_pixel),
    }
}

fn make_gray_buffer(buffer: &[u8]) -> Vec<Pixel> {
    buffer.iter().map(|&byte| Pixel::Gray(byte)).collect()
}

fn make_rgb_buffer(buffer: &[u8]) -> Vec<Pixel> {
    buffer
        .chunks(3)
        .map(|bytes| Pixel::RGB(bytes[0], bytes[1], bytes[2]))
        .collect()
}
