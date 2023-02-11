use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::OutputCallbackInfo;
use std::f32::consts::PI;
use std::io::{stdin, stdout, Write};
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::{mem, thread};

use midir::{Ignore, MidiInput};

use moroder::midi::MidiMessage;

fn main() {
    let (input_tx, input_rx) = channel();

    let input_callback = move |stamp: u64, message: &[u8], _: &mut ()| {
        input_tx.send((stamp, message.to_owned())).unwrap();

        println!("TX {}: {:?} (len = {})", stamp, message, message.len());
    };

    setup_input(input_callback);

    let block_buffer: [f32; 1024] = [0.0; 1024];
    let counter = Arc::new(Mutex::new((false, block_buffer)));
    let callback_counter = Arc::clone(&counter);
    let output_callback = move |output: &mut [f32], info: &OutputCallbackInfo| {
        let mut mutex = callback_counter.lock().unwrap();

        if !mutex.0 {
            println!("{:?}: Missed deadline", info.timestamp());
        }

        for (pos, sample) in output.iter_mut().enumerate() {
            *sample = mutex.1[pos];
        }

        mutex.0 = false;
    };

    setup_output(output_callback);

    let mut current_signal: Box<dyn Iterator<Item = f32>> = Box::new(Constant::new(0.0));

    let mut main_buffer: [f32; 1024] = [0.0; 1024];
    let main_counter = Arc::clone(&counter);
    loop {
        if let Ok(message) = input_rx.try_recv() {
            println!("RX {:?}", message);

            let message = MidiMessage::from(message.1);

            match message {
                MidiMessage::KeyDown {
                    channel,
                    key,
                    velocity,
                } => {
                    let freq = 440.0 * 2.0f32.powf((key as f32 - 69.0) / 12.0);
                    println!("start");
                    current_signal = Box::new(SineWave::new(freq));
                }
                MidiMessage::KeyUp {
                    channel,
                    key,
                    velocity,
                } => {
                    println!("stop");
                    current_signal = Box::new(Constant::new(0.0));
                }
                MidiMessage::ControllerChange {
                    channel,
                    controller,
                    value,
                } => {
                    println!("Controller {}: {}", controller, value);
                }
            }
        }

        let mut mutex = main_counter.lock().unwrap();
        if !mutex.0 {
            for sample in main_buffer.iter_mut() {
                *sample = current_signal.next().unwrap();
            }

            let _ = mem::replace(&mut mutex.1, main_buffer);

            mutex.0 = true;
        }
    }
}

fn setup_input<F>(callback: F)
where
    F: FnMut(u64, &[u8], &mut ()) + Send + 'static,
{
    thread::spawn(move || {
        let mut midi_in = MidiInput::new("midir reading input").unwrap();
        midi_in.ignore(Ignore::None);

        // Get an input port (read from console if multiple are available)
        let in_ports = midi_in.ports();
        let in_port = match in_ports.len() {
            0 => return (), //Err("no input port found".into()),
            1 => {
                println!(
                    "Choosing the only available input port: {}",
                    midi_in.port_name(&in_ports[0]).unwrap()
                );
                &in_ports[0]
            }
            _ => {
                println!("\nAvailable input ports:");
                for (i, p) in in_ports.iter().enumerate() {
                    println!("{}: {}", i, midi_in.port_name(p).unwrap());
                }
                print!("Please select input port: ");
                stdout().flush().unwrap();
                let mut input = String::new();
                stdin().read_line(&mut input).unwrap();
                in_ports
                    .get(input.trim().parse::<usize>().unwrap())
                    .ok_or("invalid input port selected")
                    .unwrap()
            }
        };

        println!("\nOpening connection");
        let in_port_name = midi_in.port_name(in_port).unwrap();

        let _conn_in = midi_in
            .connect(in_port, "midir-read-input", callback, ())
            .unwrap();

        println!(
            "Connection open, reading input from '{}' (press enter to exit) ...",
            in_port_name
        );

        loop {}
    });
}

pub fn setup_output<F>(callback: F)
where
    F: FnMut(&mut [f32], &OutputCallbackInfo) + Send + 'static,
{
    let (_host, device, config) = host_device_setup().unwrap();

    let err_fn = |err| eprintln!("Error building output sound stream: {}", err);

    thread::spawn(move || {
        let stream = device
            .build_output_stream(&config.clone().into(), callback, err_fn)
            .unwrap();

        stream.play().unwrap();

        println!(
            "Stream started playing on {} at {:?} Hz ",
            device.name().unwrap(),
            config,
        );

        loop {}
    });
}

pub fn host_device_setup(
) -> Result<(cpal::Host, cpal::Device, cpal::SupportedStreamConfig), anyhow::Error> {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .ok_or_else(|| anyhow::Error::msg("Default output device is not available"))?;

    let config = device.default_output_config()?;

    Ok((host, device, config))
}

/// An infinite source that produces a sine.
///
/// Always has a rate of 48kHz and one channel.
#[derive(Clone, Debug)]
pub struct SineWave {
    freq: f32,
    num_sample: usize,
}

impl SineWave {
    /// The frequency of the sine.
    #[inline]
    pub fn new(freq: f32) -> SineWave {
        SineWave {
            freq: freq,
            num_sample: 0,
        }
    }
}

impl Iterator for SineWave {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.num_sample = self.num_sample.wrapping_add(1);

        let value = 2.0 * PI * self.freq * self.num_sample as f32 / 44100.0;
        Some(value.sin())
    }
}

#[derive(Clone, Debug)]
pub struct Constant {
    value: f32,
}

impl Constant {
    #[inline]
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}

impl Iterator for Constant {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        Some(self.value)
    }
}
