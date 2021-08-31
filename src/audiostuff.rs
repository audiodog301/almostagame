pub struct Saw {
    pub frequency: f32,
    pub count: i32,
    pub val: f32,
}
  
impl Saw {
  #[inline]
  pub fn set_frequency(&mut self, freq: f32) {
    self.frequency = freq;
  }
  #[inline]
  pub fn next_sample(&mut self, sample_rate: f32) -> f32 {
    if self.count >= (sample_rate / self.frequency) as i32 {
      self.count = 0;
    } else {
      self.count += 1;
    }
  
      
    if self.count == 0 {
      self.val = 1.0;
    } else {
      self.val -= 1.0 / (sample_rate / self.frequency);
    }

    self.val - 0.5
  }
}