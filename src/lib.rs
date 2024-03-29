//! Abstractions for the exploration and extraction of CD-i file data.
//!
//! This crate provides a number of core abstractions for handling CD-i file data. The primary
//! abstractions are the `CdiFile` and `CdiSector` types. The `CdiFile` type represents a CD-i file
//! and provides methods for accessing the file's data and sectors. The `CdiSector` type represents
//! a single sector of a CD-i file and provides methods for accessing the sector's data and
//! subheader information.

mod data;
mod helpers;

use data::cdi_file::CdiFile;
use image::{ImageBuffer, Rgba};
use std::fs::File;
use std::io::prelude::*;

use crate::data::cdi_sector::CdiSector;
use crate::helpers::color_helpers::{read_clut_banks, read_unindexed_palette, write_palette};
use crate::helpers::image_format_helpers::{create_gif, decode_clut7_image, decode_dyuv_image, decode_rle_image, Clut7Config, DyuvImageConfig, RleImageConfig};

// test creating a cdifile
#[test]
fn test_create() {
    let file = CdiFile::new(
        "C:/Dev/Projects/Gaming/CD-i/Disc Images/Extracted/Beauty and the Beast/games.rtf"
            .to_string(),
    );
    assert_eq!(
        file.file_name(),
        "C:/Dev/Projects/Gaming/CD-i/Disc Images/Extracted/Beauty and the Beast/games.rtf"
    );
    assert_ne!(file.size(), 0);
    let audio_sectors = file.get_audio_sectors();
    let video_sectors = file.get_video_sectors();
    let data_sectors = file.get_data_sectors();

    assert_eq!(audio_sectors.len(), 13100);
    assert_eq!(video_sectors.len(), 0);
    assert_eq!(data_sectors.len(), 4955);
}

#[test]
fn test_palette_parsing() {
    let palette_file = "C:/Dev/Projects/Gaming/CD-i/FILES/testpal.bin";
    let converted_palette_file = "C:/Dev/Projects/Gaming/CD-i/FILES/testpal.png";
    let mut file = File::open(palette_file).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let palette = read_clut_banks(&buffer, 2);
    assert_eq!(palette.len(), 128);
    write_palette(converted_palette_file, &palette).unwrap();
    // read the created palette file
    let mut file = File::open(converted_palette_file).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    assert_ne!(buffer.len(), 0);
}


#[test]
fn test_dyuv_image() {
    let file = CdiFile::new(
        "C:/Dev/Projects/Gaming/CD-i/Disc Images/Extracted/Laser Lords - Nederlands/argos.rtf"
            .to_string(),
    );

    let sectors: Vec<&CdiSector> = file.get_data_sectors();
    // filter sectors to channel 7
    let channel_7_data = sectors
        .iter()
        .filter(|s| s.channel_number() == 7)
        .take(45)
        .map(|f| f.get_sector_data_by_type());

    let flattened_data: Vec<u8> = channel_7_data.clone().into_iter().flatten().collect();

    let dyuv_image = DyuvImageConfig {
        width: 384,
        height: 240,
        encoded_data: flattened_data,
        initial_y: 16,
        initial_u: 128,
        initial_v: 128,
    };
    let image = decode_dyuv_image(dyuv_image);
    assert_ne!(image.len(), 0);
    image.save("C:/Dev/Projects/Gaming/CD-i/FILES/dyuv_test.png").unwrap();
}

#[test]
fn test_clut7_image() {
    let file = CdiFile::new(
        "C:/Dev/Projects/Gaming/CD-i/Hotel Mario/L1_av.rtf"
            .to_string(),
    );

    let sectors: Vec<&CdiSector> = file.get_video_sectors();
    
    let palette_sector = file.get_data_sectors().iter().find(|s| s.sector_index() == 17).unwrap().get_sector_data_by_type();
    // get the palette data from the first 384 bytes
    let palette_data: Vec<u8> = palette_sector.iter().take(384).cloned().collect();

    let unindexed_palette = read_unindexed_palette(&palette_data);
    assert_eq!(unindexed_palette.len(), 128);

    let clut_image_sectors: Vec<&CdiSector> = sectors.iter().filter(|s| s.coding_info().video_string()== "CLUT7").take(47).cloned().collect();

    let clut_data: Vec<u8> = clut_image_sectors.iter().map(|s| s.get_sector_data_by_type()).flatten().collect();

    let clut_image = Clut7Config {
        width: 384,
        height: 280,
        encoded_data: clut_data,
        clut_data: unindexed_palette,
        use_transparency: false,
        transparency_index: 0,
        use_lower_indexes: true,
    };

    let image = decode_clut7_image(clut_image);
    assert_ne!(image.len(), 0);
    image.save("C:/Dev/Projects/Gaming/CD-i/FILES/clut7_test.png").unwrap();
}

#[test]
fn test_rle_image() {
    let file = CdiFile::new(
        "C:/Dev/Projects/Gaming/CD-i/Hotel Mario/L1_av.rtf"
            .to_string(),
    );

    let sectors: Vec<&CdiSector> = file.get_video_sectors();
    
    let palette_sector = file.get_data_sectors().iter().find(|s| s.sector_index() == 9).unwrap().get_sector_data_by_type();
    // get the palette data from the first 384 bytes
    let palette_data: Vec<u8> = palette_sector.iter().take(384).cloned().collect();

    let unindexed_palette = read_unindexed_palette(&palette_data);
    assert_eq!(unindexed_palette.len(), 128);

    let rle_image_sectors: Vec<&CdiSector> = sectors.iter().filter(|s| s.coding_info().video_string()== "RL7").take(7).cloned().collect();

    let rle_data: Vec<u8> = rle_image_sectors.iter().map(|s| s.get_sector_data_by_type()).flatten().collect();

    let rle_image = RleImageConfig {
        encoded_data: rle_data,
        line_width: 384,
        clut_data: unindexed_palette,
        use_transparency: false,
        height: 280,
    };

    let image = decode_rle_image(rle_image);
    assert_ne!(image.len(), 0);
    image.save("C:/Dev/Projects/Gaming/CD-i/FILES/rle_test.png").unwrap();
}

#[test]
fn test_gif_output() {
    let file = CdiFile::new(
        "C:/Dev/Projects/Gaming/CD-i/Disc Images/Extracted/Plunderball/Intro.rtr"
            .to_string(),
    );

    let sectors: Vec<&CdiSector> = file.get_video_sectors();
    
    let palette_sector_1 = file.get_data_sectors().iter().find(|s| s.sector_index() == 269).unwrap().get_sector_data_by_type();
    let palette_sector_2 = file.get_data_sectors().iter().find(|s| s.sector_index() == 1280).unwrap().get_sector_data_by_type();
    // get the palette data from the first 384 bytes
    let palette_data_1: Vec<u8> = palette_sector_1.iter().skip(4).take(384).cloned().collect();
    let palette_data_2: Vec<u8> = palette_sector_2.iter().skip(4).take(384).cloned().collect();

    let unindexed_palette_1 = read_unindexed_palette(&palette_data_1);
    let unindexed_palette_2 = read_unindexed_palette(&palette_data_2);
    assert_eq!(unindexed_palette_1.len(), 128);
    assert_eq!(unindexed_palette_2.len(), 128);

    let rle_image_sectors: Vec<&CdiSector> = sectors.iter().filter(|s| s.coding_info().video_string()== "RL7").cloned().collect();
    
    let mut images: Vec<ImageBuffer<Rgba<u8>, Vec<u8>>> = Vec::new();
    let mut byte_groups = Vec::new();
    
    for (index, sector) in rle_image_sectors.iter().enumerate() {
        let rle_data: Vec<u8> = sector.get_sector_data_by_type();
        byte_groups.push(rle_data);

        if sector.submode().is_trigger() {
            let palette = if sector.sector_index() >= 1280 {
                &unindexed_palette_2
            } else {
                &unindexed_palette_1
            };
            let data = byte_groups.iter().flatten().cloned().collect();
            let rle_image = RleImageConfig {
                encoded_data: data,
                line_width: 384,
                clut_data: palette.to_vec(),
                use_transparency: false,
                height: 240,
            };
            if let image = decode_rle_image(rle_image) {
                images.push(image);
            }
            byte_groups.clear();
        }
    }

    create_gif(images, "C:/Dev/Projects/Gaming/CD-i/FILES/plunderball_intro.gif",384,280).unwrap();
    
}
