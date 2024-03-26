struct CdiSubModeInfo {
    coding_info: u8,
    byte: u8,
    channel: u8,
}

impl SubModeInfo {
    pub fn new(b: u8, channel: u8, coding_info: u8) -> Self {
        SubModeInfo {
            byte: b,
            channel,
            coding_info,
        }
    }

    pub fn is_empty_sector(&self) -> bool {
        (self.byte & SubModeBit::Any as u8) == 0 && self.channel == 0 && self.coding_info == 0
    }

    pub fn is_eof(&self) -> bool {
        (self.byte & (1 << 7)) != 0
    }

    pub fn is_rtf(&self) -> bool {
        (self.byte & (1 << 6)) != 0
    }

    pub fn is_form2(&self) -> bool {
        (self.byte & (1 << 5)) != 0
    }

    pub fn is_trigger(&self) -> bool {
        (self.byte & (1 << 4)) != 0
    }

    pub fn is_data(&self) -> bool {
        (self.byte & (1 << 3)) != 0
    }

    pub fn is_audio(&self) -> bool {
        (self.byte & (1 << 2)) != 0
    }

    pub fn is_video(&self) -> bool {
        (self.byte & (1 << 1)) != 0
    }

    pub fn is_eor(&self) -> bool {
        (self.byte & 1) != 0
    }
}
