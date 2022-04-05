#[derive(Clone, Debug)]
pub struct SineWave {
    frequency: f32,
    num_sample: u32,
}

impl Iterator for SineWave {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.num_sample = self.num_sample.wrapping_add(1);

        if self.num_sample == SAMPLE_RATE {
            self.num_sample = 0;
        }

        let value = 2.0 * PI * self.frequency * self.num_sample as f32 / SAMPLE_RATE as f32;
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
