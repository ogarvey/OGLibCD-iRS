#[derive(Clone, Copy)]
pub struct CdiCodingInfo {
  byte: u8,
}

impl CdiCodingInfo {
  pub fn new(b: u8) -> Self {
    CdiCodingInfo { byte: b }
  }
  
  // Audio Properties
  pub fn emphasis(&self) -> bool {
    self.byte >> 6 & 0b1 == 1
  }

  pub fn bits_per_sample(&self) -> u8 {
    self.byte >> 4 & 0b11
  }

  pub fn bits_per_sample_string(&self) -> String {
    match self.bits_per_sample() {
      0 => "4-bit".to_string(),
      1 => "8-bit".to_string(),
      _ => "Reserved".to_string(),
    }
  }

  pub fn bits_per_sample_value(&self) -> u8 {
    match self.bits_per_sample() {
      0 => 4,
      1 => 8,
      _ => 0,
    }
  }

  pub fn sample_rate(&self) -> u8 {
    self.byte >> 2 & 0b11
  }

  pub fn sample_rate_string(&self) -> String {
    match self.sample_rate() {
      0 => "37.8 kHz".to_string(),
      1 => "18.9 kHz".to_string(),
      _ => "Reserved".to_string(),
    }
  }

  pub fn sample_rate_value(&self) -> u32 {
    match self.sample_rate() {
      0 => 37800,
      1 => 18900,
      _ => 0,
    }
  }

  pub fn is_stereo(&self) -> bool {
    self.byte & 0b11 == 1
  }

  pub fn is_mono(&self) -> bool {
    self.byte & 0b11 == 0
  }

  // Video Properties
  pub fn is_ascf(&self) -> bool {
    self.byte >> 7 & 0b1 == 1
  }

  pub fn is_odd_lines(&self) -> bool {
    self.byte >> 6 & 0b1 == 1
  }

  pub fn resolution(&self) -> u8 {
    self.byte >> 4 & 0b11
  }

  pub fn resolution_string(&self) -> String {
    match self.resolution() {
      0 => "Normal".to_string(),
      1 => "Double".to_string(),
      2 => "Reserved".to_string(),
      3 => "High".to_string(),
      _ => "Reserved".to_string(),
    }
  }

  pub fn coding(&self) -> u8 {
    self.byte & 0b1111
  }

  pub fn video_string(&self) -> String {
    match self.coding() {
      0 => "CLUT4".to_string(),
      1 => "CLUT7".to_string(),
      2 => "CLUT8".to_string(),
      3 => "RL3".to_string(),
      4 => "RL7".to_string(),
      5 => "DYUV".to_string(),
      6 => "RGB555L".to_string(),
      7 => "RGB555H".to_string(),
      8 => "QHY".to_string(),
      15 => "MPEG".to_string(),
      _ => "Reserved".to_string(),
    }
  }
}
