#[derive(Debug)]
pub enum Pixel {
    Gray(u8),
    BGR(u8, u8, u8),
}

pub fn make_buffer(buffer: &[u8], bits_per_pixel: usize, width: usize) -> Vec<Pixel> {
    match bits_per_pixel {
        8 => make_gray_buffer(buffer),
        24 => make_rgb_buffer(buffer, width),
        _ => unimplemented!("{} bits per pixel", bits_per_pixel),
    }
}

fn make_gray_buffer(buffer: &[u8]) -> Vec<Pixel> {
    buffer.iter().map(|&byte| Pixel::Gray(byte)).collect()
}

fn make_rgb_buffer(buffer: &[u8], width: usize) -> Vec<Pixel> {
    let height = buffer.len() / width / 3;
    let padding = if (width * 3).rem_euclid(4) == 0 {
        0
    } else {
        4 - (width * 3).rem_euclid(4)
    };
    let mut iter = buffer.iter();
    let mut content = Vec::new();

    for _ in 0..height {
        for _ in 0..width {
            if let (Some(&r), Some(&g), Some(&b)) = (iter.next(), iter.next(), iter.next()) {
                content.push(Pixel::BGR(r, g, b));
            }
        }

        // Ignoring padding bytes
        for _ in 0..padding {
            iter.next();
        }
    }

    content
}
