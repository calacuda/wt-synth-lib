#![no_std]

pub const WAVE_TABLE_SIZE: usize = 128;

#[derive(Clone, Copy, Debug)]
pub struct Overtone {
    /// the frequency of the overtone relative to the fundimental
    pub overtone: f64,
    /// how loud this over tone is relative to the total volume (ie, 1.0)
    pub volume: f64,
}

pub struct WavetableOscillator {
    sample_rate: u32,
    wave_table: [f32; WAVE_TABLE_SIZE],
    index: f32,
    index_increment: f32,
}

impl WavetableOscillator {
    pub fn new(sample_rate: u32, overtones: [Option<Overtone>; 10]) -> Self {
        Self {
            sample_rate,
            wave_table: Self::build_wave_table(overtones),
            index: 0.0,
            index_increment: 0.0,
        }
    }

    fn build_wave_table(overtones: [Option<Overtone>; 10]) -> [f32; WAVE_TABLE_SIZE] {
        let mut wave_table = [0.0; WAVE_TABLE_SIZE];

        let mut n_overtones = 0;

        for ot in overtones {
            if ot.is_some() {
                n_overtones += 1;
            }
        }

        let bias = 1.0 / n_overtones as f32;

        for i in 0..WAVE_TABLE_SIZE {
            for ot in overtones {
                if let Some(ot) = ot {
                    wave_table[i] += (libm::sin(
                        2.0 * core::f64::consts::PI * i as f64 * ot.overtone
                            / WAVE_TABLE_SIZE as f64,
                    ) * ot.volume) as f32
                }
            }

            wave_table[i] *= bias;
        }

        wave_table
    }

    pub fn set_overtones(&mut self, overtones: [Option<Overtone>; 10]) {
        self.wave_table = Self::build_wave_table(overtones);
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.index_increment = frequency * self.wave_table.len() as f32 / self.sample_rate as f32;
    }

    pub fn get_sample(&mut self) -> f32 {
        let sample = self.lerp();
        self.index += self.index_increment;
        self.index %= self.wave_table.len() as f32;
        sample
    }

    fn lerp(&self) -> f32 {
        let truncated_index = self.index as usize;
        let next_index = (truncated_index + 1) % self.wave_table.len();

        let next_index_weight = self.index - truncated_index as f32;
        let truncated_index_weight = 1.0 - next_index_weight;

        truncated_index_weight * self.wave_table[truncated_index]
            + next_index_weight * self.wave_table[next_index]
    }
}

impl Iterator for WavetableOscillator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        return Some(self.get_sample());
    }
}
