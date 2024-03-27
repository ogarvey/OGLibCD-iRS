use crate::data::cdi_sector::CdiSector;
use std::fs;

pub struct CdiFile {
    pub file_name: String,
    data: Vec<u8>,
    sectors: Vec<CdiSector>,
}

impl CdiFile {
  const SECTOR_SIZE: u64 = 2352;
    pub fn new(file_name: String) -> Self {
        let data = fs::read(&file_name).unwrap();
        let mut sectors = Vec::<CdiSector>::new();
        let mut parsed: usize = 0;
        let mut sector_count = 0;
        while parsed < data.len() {
            let sector = CdiSector::new(sector_count, data.iter().skip(parsed).take(Self::SECTOR_SIZE as usize).cloned().collect());
            sectors.push(sector);
            parsed += Self::SECTOR_SIZE as usize;
            sector_count += 1;
        }
        CdiFile { file_name, data, sectors }
    }

    pub fn file_name(&self) -> &String {
        &self.file_name
    }

    pub fn size(&self) -> u64 {
        self.data.len() as u64
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn sectors(&self) -> &Vec<CdiSector> {
        &self.sectors
    }

    pub fn sector(&self, index: usize) -> &CdiSector {
        &self.sectors[index]
    }

    pub fn sector_count(&self) -> usize {
        self.sectors.len()
    }

    pub fn sector_data(&self, index: usize) -> &Vec<u8> {
        &self.sectors[index].sector_data
    }

    pub fn get_video_sectors(&self) -> Vec<&CdiSector> {
        self.sectors.iter().filter(|s| s.submode_info.is_video()).collect()
    }

    pub fn get_audio_sectors(&self) -> Vec<&CdiSector> {
        self.sectors.iter().filter(|s| s.submode_info.is_audio()).collect()
    }

    pub fn get_data_sectors(&self) -> Vec<&CdiSector> {
        self.sectors.iter().filter(|s| s.submode_info.is_data()).collect()
    }
}
