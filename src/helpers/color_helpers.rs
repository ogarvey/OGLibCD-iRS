use image::{ImageBuffer, Rgba};

/// Reads unindexed palette bytes and returns the colours as `Vec<Rgba<u8>>`.
///
/// Reads a palette represented by 3 bytes per colour and returns a 
/// vector of `Rgba<u8>` colours at full opacity.
pub fn read_unindexed_palette(data: &[u8]) -> Vec<Rgba<u8>> {
    let mut colors = Vec::new();
    let length = data.len();

    for i in (0..length).step_by(3) {
        let color = Rgba([data[i], data[i + 1], data[i + 2], 255]);
        colors.push(color);
    }

    colors
}

/// Reads indexed palette bytes and returns the colours as `Vec<Rgba<u8>>`.
///
/// Reads a palette represented by a 1 byte index, and 3 bytes per colour
/// and returns a vector of `Rgba<u8>` colours at full opacity.
pub fn read_indexed_palette(data: &[u8]) -> Vec<Rgba<u8>> {
    let mut colors = Vec::new();
    let length = data.len();

    for i in (0..length).step_by(4) {
        let color = Rgba([data[i + 1], data[i + 2], data[i + 3], 255]);
        colors.push(color);
    }

    colors
}

/// Reads CLUT palette bytes and returns the colours as `Vec<Rgba<u8>>`.
///
/// Reads a CLUT marked palette represented by a marker of `0xC30000??`,
/// where `??` can be one of `00`, `01`, `02`, `03`,
/// followed by a series of 4 byte groups consisting of a 1 byte index, 
/// and 3 bytes per colour and returns a vector of `Rgba<u8>` colours at full opacity.
pub fn read_clut_banks(data: &[u8], count: u8) -> Vec<Rgba<u8>> {
    let mut clut_bank_colors = Vec::new();
    let length = data.len();
    let bank_length = 0x100;

    for i in 0..count as usize {
        for j in (4..bank_length + 4).step_by(4) {
            let index = i * bank_length + (i * 4);
            if index + j + 3 < length {
                let color = Rgba([
                    data[index + j + 1],
                    data[index + j + 2],
                    data[index + j + 3],
                    255,
                ]);
                clut_bank_colors.push(color);
                println!("{color:?}");
            }
        }
    }

    clut_bank_colors
}

/// Writes a `Vec<Rgba<u8>>` palette to a png file.
///
/// Writes a palette represented by a vector of `Rgba<u8>` colours,
/// with each colour represented by an 8x8 pixel square, to a png file at the specified path.
pub fn write_palette(path: &str, colors: &[Rgba<u8>]) -> Result<(), Box<dyn std::error::Error>> {
    // Create a new RGBA image buffer with dimensions 256x256
    let mut palette_img = ImageBuffer::new(256, 256);

    // Iterate over the colors and draw 8x8 pixel squares for each color
    let mut x = 0;
    let mut y = 0;
    let width = 8;
    let height = 8;
    for color in colors {
        // Fill a rectangle with the current color
        for dx in 0..width {
            for dy in 0..height {
                palette_img.put_pixel(x + dx, y + dy, *color);
            }
        }

        // Update x and y positions
        x += width;
        if x >= 256 {
            x = 0;
            y += height;
        }
    }

    // Save the image to the specified path as PNG format
    palette_img.save(path)?;

    Ok(())
}
