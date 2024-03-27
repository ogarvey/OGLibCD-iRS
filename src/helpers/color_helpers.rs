use image::{ImageBuffer, Rgba, ImageFormat};

pub fn read_unindexed_palette(data: &[u8]) -> Vec<Rgba<u8>> {
    let mut colors = Vec::new();
    let length = data.len();
    
    for i in (0..length).step_by(3) {
        if i + 3 < length {
            let color = Rgba([data[i ], data[i + 1], data[i + 2], 255]);
            colors.push(color);
        }
    }
    
    colors
}
pub fn read_indexed_palette(data: &[u8]) -> Vec<Rgba<u8>> {
    let mut colors = Vec::new();
    let length = data.len();
    
    for i in (0..length).step_by(4) {
        if i + 3 < length {
            let color = Rgba([data[i + 1], data[i + 2], data[i + 3], 255]);
            colors.push(color);
        }
    }
    
    colors
}

pub fn read_clut_banks(data: &[u8], count: u8) -> Vec<Rgba<u8>> {
    let mut clut_bank_colors = Vec::new();
    let length = data.len();
    let bank_length = 0x100;
    
    for i in 0..count as usize {
        let mut colors = Vec::new();
        for j in (4..bank_length+4).step_by(4) {
            if (i*bank_length + (i*4)) + j + 3 < length {
                let color = Rgba([data[(i*bank_length + (i*4)) + j + 1], data[(i*bank_length + (i*4)) + j + 2], data[(i*bank_length + (i*4)) + j + 3], 255]);
                colors.push(color);
                println!("{:?}", color);
            }
        }
        clut_bank_colors.extend(colors);
    }
    
    clut_bank_colors
}

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
