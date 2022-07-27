#[derive(Debug)]
pub enum Pixel {
    Gray(u8),
    BGR(u8, u8, u8),
}

pub fn make_buffer(
    buffer: &[u8],
    bits_per_pixel: usize,
    width: usize,
    height: usize,
    padding: usize,
) -> Vec<Pixel> {
    match bits_per_pixel {
        8 => make_gray_buffer(buffer),
        24 => make_rgb_buffer(buffer, width, height, padding),
        _ => unimplemented!("{} bits per pixel", bits_per_pixel),
    }
}

fn make_gray_buffer(buffer: &[u8]) -> Vec<Pixel> {
    buffer.iter().map(|&byte| Pixel::Gray(byte)).collect()
}

fn make_rgb_buffer(buffer: &[u8], width: usize, height: usize, padding: usize) -> Vec<Pixel> {
    let mut iter = buffer.iter();
    let mut content = Vec::new();

    for _ in 0..height {
        for _ in 0..width {
            if let (Some(&b), Some(&g), Some(&r)) = (iter.next(), iter.next(), iter.next()) {
                content.push(Pixel::BGR(b, g, r));
            }
        }

        // Ignoring padding bytes
        for _ in 0..padding {
            iter.next();
        }
    }

    content
}
