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
use std::fs::File;
use std::io::prelude::*;

use crate::data::cdi_sector::CdiSector;
use crate::helpers::color_helpers::{read_clut_banks, write_palette};
use crate::helpers::image_format_helpers::{decode_dyuv_image, DyuvImageConfig};

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
    // write the channel 7 data to a file

    let flattened_data: Vec<u8> = channel_7_data.clone().into_iter().flatten().collect();

    let mut dyuv_image = DyuvImageConfig {
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
