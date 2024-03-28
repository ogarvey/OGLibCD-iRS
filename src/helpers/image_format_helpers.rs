use std::u32;

use image::{ImageBuffer, Rgba};

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

            let encoded_pixel = ((encoded_data[encoded_index] as u16) << 8) | (encoded_data[encoded_index + 1] as u16);
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

            decoded_image.put_pixel(x, y, Rgba([rgb1[0] as u8, rgb1[1] as u8, rgb1[2] as u8, 0xFF]));
            if x + 1 < config.width {
                decoded_image.put_pixel(x + 1, y, Rgba([rgb2[0] as u8, rgb2[1] as u8, rgb2[2] as u8, 0xFF]));
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
