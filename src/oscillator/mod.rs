use std::{f32::consts::PI, time::Duration};

use rodio::{Sample, Source};

use crate::SAMPLE_RATE;

// TODO: tests
pub mod sine_wave;

pub struct Oscillator {}

impl Oscillator {
    pub fn sine_wave(frequency: f32) -> SineWave {
        SineWave {
            frequency,
            num_sample: 0,
        }
    }

    pub fn square_wave(frequency: f32) -> SquareWave {
        SquareWave {
            frequency,
            num_sample: 0,
        }
    }

    pub fn sawtooth_wave(frequency: f32) -> SawtoothWave {
        SawtoothWave {
            frequency,
            num_sample: 0,
        }
    }

    pub fn triangle_wave(frequency: f32) -> TriangleWave {
        TriangleWave {
            frequency,
            num_sample: 0,
        }
    }
}

trait Wave {}

#[derive(Clone, Debug)]
pub struct SquareWave {
    frequency: f32,
    num_sample: u32,
}

impl Iterator for SquareWave {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.num_sample = self.num_sample.wrapping_add(1);

        if self.num_sample == SAMPLE_RATE {
            self.num_sample = 0;
        }

        let value = 2.0 * PI * self.frequency * self.num_sample as f32 / SAMPLE_RATE as f32;

        if value.sin() > 0.0 {
            Some(1.0)
        } else {
            Some(-1.0)
        }
    }
}

impl Source for SquareWave {
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

#[derive(Clone, Debug)]
pub struct SawtoothWave {
    frequency: f32,
    num_sample: u32,
}

impl Iterator for SawtoothWave {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.num_sample = self.num_sample.wrapping_add(1);

        if self.num_sample == SAMPLE_RATE {
            self.num_sample = 0;
        }

        let value = self.frequency * self.num_sample as f32 / SAMPLE_RATE as f32;

        Some(2.0 * (value - ((1.0 / 2.0) + value).floor()))
    }
}

impl Source for SawtoothWave {
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

#[derive(Clone, Debug)]
pub struct TriangleWave {
    frequency: f32,
    num_sample: u32,
}

impl Iterator for TriangleWave {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.num_sample = self.num_sample.wrapping_add(1);

        if self.num_sample == SAMPLE_RATE {
            self.num_sample = 0;
        }

        let value = self.frequency * self.num_sample as f32 / SAMPLE_RATE as f32;

        Some(2.0 * (value - ((1.0 / 2.0) + value).floor()).abs())
    }
}

impl Source for TriangleWave {
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
