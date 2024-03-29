use std::{fs::File, io::BufWriter, u32};

use image::{ImageBuffer, Rgba, imageops::FilterType};
use gif::{Frame, Encoder, Repeat};

const dequantizer_array: [u8; 16] = [
    0, 1, 4, 9, 16, 27, 44, 79, 128, 177, 212, 229, 240, 247, 252, 255,
];

pub struct DyuvImageConfig {
    pub width: u32,
    pub height: u32,
    pub encoded_data: Vec<u8>,
    pub initial_y: u32,
    pub initial_u: u32,
    pub initial_v: u32,
}

pub fn decode_dyuv_image(config: DyuvImageConfig) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut encoded_index: usize = 0;
    let mut decoded_image = ImageBuffer::new(config.width, config.height);
    let encoded_data = &config.encoded_data[..];

    for y in 0..config.height {
        let mut prev_y = config.initial_y as u16;
        let mut prev_u = config.initial_u as u16;
        let mut prev_v = config.initial_v as u16;

        for x in (0..config.width).step_by(2) {
            if encoded_index >= encoded_data.len() || encoded_index + 1 >= encoded_data.len() {
                break;
            }

            let encoded_pixel = ((encoded_data[encoded_index] as u16) << 8)
                | (encoded_data[encoded_index + 1] as u16);
            let du1 = ((encoded_pixel & 0xF000) >> 12) as u8;
            let dy1 = ((encoded_pixel & 0x0F00) >> 8) as u8;
            let dv1 = ((encoded_pixel & 0x00F0) >> 4) as u8;
            let dy2 = (encoded_pixel & 0x000F) as u8;

            let yout1 = ((prev_y + dequantizer_array[dy1 as usize] as u16) % 256) as u8;
            let uout2 = ((prev_u + dequantizer_array[du1 as usize] as u16) % 256) as u8;
            let vout2 = ((prev_v + dequantizer_array[dv1 as usize] as u16) % 256) as u8;
            let yout2 = ((yout1 as u16 + dequantizer_array[dy2 as usize] as u16) % 256) as u8;

            let uout1 = ((prev_u + uout2 as u16) / 2) as u8;
            let vout1 = ((prev_v + vout2 as u16) / 2) as u8;

            prev_y = yout2 as u16;
            prev_u = uout2 as u16;
            prev_v = vout2 as u16;

            let rgb1 = yuv_to_rgb(yout1.into(), uout1.into(), vout1.into());
            let rgb2 = yuv_to_rgb(yout2.into(), uout2.into(), vout2.into());

            decoded_image.put_pixel(
                x,
                y,
                Rgba([rgb1[0] as u8, rgb1[1] as u8, rgb1[2] as u8, 0xFF]),
            );
            if x + 1 < config.width {
                decoded_image.put_pixel(
                    x + 1,
                    y,
                    Rgba([rgb2[0] as u8, rgb2[1] as u8, rgb2[2] as u8, 0xFF]),
                );
            }

            encoded_index += 2;
        }
    }

    decoded_image
}

fn yuv_to_rgb(y: i32, u: i32, v: i32) -> Rgba<u8> {
    let r = clamp((y * 256 + 351 * (v - 128)) / 256) as u8;
    let g = clamp(((y * 256) - (86 * (u - 128) + 179 * (v - 128))) / 256) as u8;
    let b = clamp((y * 256 + 444 * (u - 128)) / 256) as u8;
    Rgba([r, g, b, 255])
}

fn clamp(value: i32) -> i32 {
    value.clamp(0, 255)
}

pub struct Clut7Config {
    pub width: u32,
    pub height: u32,
    pub encoded_data: Vec<u8>,
    pub clut_data: Vec<Rgba<u8>>,
    pub use_transparency: bool,
    pub transparency_index: u8,
    pub use_lower_indexes: bool,
}

pub fn decode_clut7_image(config: Clut7Config) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut decoded_image = ImageBuffer::new(config.width, config.height);
    let encoded_data = &config.encoded_data[..];
    let clut_data = &config.clut_data[..];

    for y in 0..config.height {
        for x in 0..config.width {
            let index = (y * config.width + x) as usize;
            let clut_index = encoded_data[index] as usize;
            let color = if clut_index < clut_data.len() {
                if config.use_transparency
                    && ((config.use_lower_indexes
                        && clut_index <= config.transparency_index as usize)
                        || (!config.use_lower_indexes
                            && clut_index >= config.transparency_index as usize)
                        || clut_index == 0)
                {
                    Rgba([0, 0, 0, 0])
                } else {
                    clut_data[clut_index]
                }
            } else {
                clut_data[clut_index % clut_data.len()]
            };
            decoded_image.put_pixel(x, y, color);
        }
    }
    decoded_image
}

pub fn decode_rle_bytes(rle_data: &[u8], line_width: usize) -> Vec<u8> {
    let mut lines = Vec::new();
    let mut current_line = Vec::new();

    let mut i = 0;
    while i < rle_data.len() {
        let first_byte = rle_data[i];
        let is_run = (first_byte & 0x80) != 0; // Check if the MSB is set
        let color_index = first_byte & 0x7F; // Extract color index (7 bits)

        if is_run {
            if i + 1 >= rle_data.len() {
                break;
            }

            let run_length = rle_data[i + 1] as usize;
            i += 2;

            if run_length == 1 {
                continue; // Skipping run length of 1 as per the original logic
            }

            let add_length = if run_length == 0 {
                line_width - current_line.len()
            } else {
                run_length
            };

            let actual_add_length = std::cmp::min(add_length, line_width - current_line.len());
            current_line.extend(std::iter::repeat(color_index).take(actual_add_length));
        } else {
            // Single pixel
            current_line.push(color_index);
            i += 1;
        }

        if current_line.len() == line_width {
            lines.extend(current_line.drain(..));
        }
    }

    // Add the last line if not empty
    if !current_line.is_empty() {
        lines.extend(current_line);
    }

    lines
}

pub struct RleImageConfig {
    pub encoded_data: Vec<u8>,
    pub line_width: usize,
    pub height: usize,
    pub clut_data: Vec<Rgba<u8>>,
    pub use_transparency: bool,
}

pub fn decode_rle_image(config: RleImageConfig) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
  let image_bytes = decode_rle_bytes(&config.encoded_data, config.line_width);
  let height = if config.height == 0 {
    image_bytes.len() / config.line_width
  } else {
    config.height
  };
  let clut_config = Clut7Config {
    width: config.line_width as u32,
    height: config.height as u32,
    encoded_data: image_bytes,
    clut_data: config.clut_data.clone(),
    use_transparency: config.use_transparency,
    transparency_index: 0,
    use_lower_indexes: false,
  };
  decode_clut7_image(clut_config)
}

pub fn create_gif(images: Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>, path: &str, width: u16, height: u16) -> Result<(), Box<dyn std::error::Error>> {
    let mut image_output = File::create(path)?;
    let mut encoder = Encoder::new(BufWriter::new(&mut image_output), width, height, &[])?;
    encoder.set_repeat(Repeat::Infinite)?;

    for image in images {
        let resized = image::imageops::resize(&image, width.into(), height.into(), FilterType::Nearest);
        
        let mut frame = Frame::from_rgba_speed(width, height, &mut resized.into_raw(), 10);
        frame.delay = 10; // Set the delay between frames (in 1/100th of a second)
        encoder.write_frame(&frame)?;
    }

    Ok(())
}
