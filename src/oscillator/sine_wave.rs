use std::{f32::consts::PI, time::Duration};

use rodio::Source;

use crate::SAMPLE_RATE;

#[derive(Clone, Debug)]
pub struct SineWave {
    frequency: f32,
    num_sample: u32,
}

impl SineWave {
    pub fn new(frequency: f32) -> Self {
        SineWave {
            frequency,
            num_sample: 0,
        }
    }
}

impl Iterator for SineWave {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        if self.num_sample == SAMPLE_RATE {
            self.num_sample = 0;
        }

        let value = 2.0 * PI * self.frequency * self.num_sample as f32 / SAMPLE_RATE as f32;

        self.num_sample = self.num_sample.wrapping_add(1);

        Some(value.sin())
    }
}

impl Source for SineWave {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> u16 {
        1
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        SAMPLE_RATE as u32
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use rodio::Source;

    use super::SineWave;

    #[test]
    fn sine_wave() {
        let frequency = 1440.0;

        let sine_wave = SineWave::new(frequency);
        let sample_rate = sine_wave.sample_rate();

        let mut n = 0;
        for sample in sine_wave {
            let expected_value = (2.0 * PI * frequency * n as f32 / sample_rate as f32).sin();

            assert_eq!(expected_value, sample);

            if n > 1000 {
                break;
            }

            n = n + 1;
        }
    }
}
