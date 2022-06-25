pub mod sawtooth_wave;
pub mod sine_wave;
pub mod square_wave;
pub mod triangle_wave;

use self::{
    sawtooth_wave::SawtoothWave, sine_wave::SineWave, square_wave::SquareWave,
    triangle_wave::TriangleWave,
};

pub struct Oscillator {}

impl Oscillator {
    pub fn sine_wave(frequency: f32) -> SineWave {
        SineWave::new(frequency)
    }

    pub fn square_wave(frequency: f32) -> SquareWave {
        SquareWave::new(frequency)
    }

    pub fn sawtooth_wave(frequency: f32) -> SawtoothWave {
        SawtoothWave::new(frequency)
    }

    pub fn triangle_wave(frequency: f32) -> TriangleWave {
        TriangleWave::new(frequency)
    }
}
