use crate::data::cdi_coding_info::CdiCodingInfo;
use crate::data::cdi_submode_info::CdiSubModeInfo;
use crate::data::CdiSectorType;
use crate::data::CdiSubHeaderByte;

pub struct CdiSector {
  pub sector_index: u32,
  pub sector_data: Vec<u8>,
  pub sector_sub_header_data: Vec<u8>,
  pub coding_info: CdiCodingInfo,
  pub submode_info: CdiSubModeInfo,
}

impl CdiSector {
  const HEADER_SIZE: u32 = 16;
  const SUBHEADER_SIZE: u32 = 8;

  const SECTOR_SIZE: u32 = 2352;
  const SECTOR_DATA_SIZE: u32 = 2048;
  const SECTOR_VIDEO_SIZE: u32 = 2324;
  const SECTOR_AUDIO_SIZE: u32 = 2304;

  pub fn new(sector_index: u32, sector_data: Vec<u8>) -> Self {
    let coding_info = CdiCodingInfo::new(sector_data[(Self::HEADER_SIZE + (CdiSubHeaderByte::CodingInfo as u32)) as usize]);
    let submode_info = CdiSubModeInfo::new(sector_data[(Self::HEADER_SIZE + (CdiSubHeaderByte::Submode as u32)) as usize],sector_data[(Self::HEADER_SIZE + (CdiSubHeaderByte::ChannelNumber as u32) )as usize],sector_data[(Self::HEADER_SIZE + (CdiSubHeaderByte::CodingInfo as u32)) as usize]);
    let sub_header_data = sector_data.iter().skip(Self::HEADER_SIZE as usize).take(Self::SUBHEADER_SIZE as usize).cloned().collect();
    CdiSector {
      sector_index,
      sector_data,
      sector_sub_header_data: sub_header_data,
      coding_info,
      submode_info,
    }
  }

  pub fn sector_index(&self) -> u32 {
    self.sector_index
  }

  pub fn file_number(&self) -> u8 {
    self.sector_sub_header_data[CdiSubHeaderByte::FileNumber as usize]
  }

  pub fn channel_number(&self) -> u8 {
    self.sector_sub_header_data[CdiSubHeaderByte::ChannelNumber as usize]
  }

  pub fn submode(&self) -> CdiSubModeInfo {
    self.submode_info
  }

  pub fn coding_info(&self) -> CdiCodingInfo {
    self.coding_info
  }

  pub fn get_sector_type(&self) -> CdiSectorType {
    if self.submode_info.is_audio() {
      return CdiSectorType::Audio;
    } else if self.submode_info.is_video() {
      return CdiSectorType::Video;
    } else if self.submode_info.is_data() {
      return CdiSectorType::Data;
    } else {
      return CdiSectorType::Empty;
    }
  }

  pub fn get_sector_data_by_type(&self) -> Vec<u8> {
    let start_offset: u32 = (Self::HEADER_SIZE+(Self::SUBHEADER_SIZE)).into();
    match self.get_sector_type() {
      CdiSectorType::Video => self.sector_data.iter().skip(start_offset as usize).take(Self::SECTOR_VIDEO_SIZE as usize).cloned().collect::<Vec<u8>>(),
      CdiSectorType::Data => self.sector_data.iter().skip(start_offset as usize).take(Self::SECTOR_DATA_SIZE as usize).cloned().collect::<Vec<u8>>(),
      CdiSectorType::Audio => self.sector_data.iter().skip(start_offset as usize).take(Self::SECTOR_AUDIO_SIZE as usize).cloned().collect::<Vec<u8>>(),
      _ => self.sector_data.iter().skip(start_offset as usize).take(Self::SECTOR_DATA_SIZE as usize).cloned().collect::<Vec<u8>>(),
    }
  }
}
