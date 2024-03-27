#![allow(dead_code)]
pub mod cdi_file;
pub mod cdi_sector;
pub mod cdi_coding_info;
pub mod cdi_submode_info;

pub enum CdiPaletteType {
    RGB,
    Indexed,
    Clut,
}

pub enum CdiSectorType {
    Data,
    Audio,
    Video,
    Empty,
    Message,
}

pub enum CdiVideoType {
    CLUT4,
    CLUT7,
    CLUT8,
    RL3,
    RL7,
    DYUV,
    RGB555L,
    RGB555H,
    QHY,
    MPEG = 0xF,
    Reserved,
}

pub enum CdiSubHeaderByte {
    FileNumber,
    ChannelNumber,
    Submode,
    CodingInfo,
}

#[derive(Copy, Clone)]
pub enum SubModeBit {
    EOR = 0b00000001,
    Video = 0b00000010,
    Audio = 0b00000100,
    Data = 0b00001000,
    Trigger = 0b00010000,
    Form = 0b00100000,
    RealTime = 0b01000000,
    EOF = 0b10000000,
    Any = 0b00001110,
}
