use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::thread;

use crate::gamestuff::PlayerDetails;
use crate::instructions::Instruction;

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

pub fn cpal_stuff(receiver: crossbeam_channel::Receiver<Instruction>) {
    let mut children = Vec::new();
    children.push(thread::spawn( move ||  {
        #[cfg(all(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd"),feature = "jack"))]
        // Manually check for flags. Can be passed through cargo with -- e.g.
        // cargo run --release --example beep --features jack -- --jack
        let host = if std::env::args()
            .collect::<String>()
            .contains(&String::from("--jack"))
        {
            cpal::host_from_id(cpal::available_hosts()
                .into_iter()
                .find(|id| *id == cpal::HostId::Jack)
                .expect(
                    "make sure --features jack is specified. only works on OSes where jack is available",
                )).expect("jack host unavailable")
        } else {
            cpal::default_host()
        };

        #[cfg(any(not(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd")),not(feature = "jack")))]
        let host = cpal::default_host();

        let device = host
            .default_output_device()
            .expect("failed to find a default output device");
        let config = device.default_output_config().unwrap();

        match config.sample_format() {
            cpal::SampleFormat::F32 => run::<f32>(&device, &config.into(), receiver),
            cpal::SampleFormat::I16 => run::<i16>(&device, &config.into(), receiver),
            cpal::SampleFormat::U16 => run::<u16>(&device, &config.into(), receiver),
        };
    }));
}

fn run<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    receiver: crossbeam_channel::Receiver<Instruction>,
) where
    T: cpal::Sample,
{
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let mut saw: Saw = Saw {
        frequency: 220.0,
        count: 0,
        val: 0.0,
    };

    let mut vol = 0.0;

    let stream = device
        .build_output_stream(
            config,
            move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                for frame in data.chunks_mut(channels) {
                    while let Ok(instruction) = receiver.try_recv() {
                        match instruction {
                            Instruction::Volume(v) => vol = v,
                            Instruction::Pitch(p) => saw.set_frequency(p.abs() * 2.5),
                        }
                    }

                    let value: T =
                        cpal::Sample::from::<f32>(&(saw.next_sample(44_100.0) * vol * 0.3));
                    for sample in frame.iter_mut() {
                        *sample = value;
                    }
                }
            },
            err_fn,
        )
        .unwrap();
    stream.play();

    loop {}
}

pub fn process_player_details(player_details: PlayerDetails) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    if player_details.clicking == true {
        instructions.push(Instruction::Volume(1.0));
    } else {
        instructions.push(Instruction::Volume(0.0));
    }

    instructions.push(Instruction::Pitch(player_details.pos.x * 100.0));

    instructions
}

pub struct Connection {
    val: f32,
    to_set: Vec<f32>,
}
impl Connection {
    pub fn new() -> Self {
        Self {
            val: 0.0,
            to_set: Vec::new(),
        }
    }
    pub fn get_value(&self) -> f32 {
        self.val
    }
    pub fn set_value(&mut self, val: f32) {
        self.to_set.push(val);
    }
    pub fn update(&mut self) {
        if !self.to_set.is_empty() {
            self.val = self.to_set.iter().sum::<f32>() / self.to_set.len() as f32;
            self.to_set.clear();
        }
    }
}

pub struct FauxGraph {
    connections: [Connection; 8],
}
impl FauxGraph {
    pub fn new() -> Self {
        Self {
            connections: [
                Connection::new(),
                Connection::new(),
                Connection::new(),
                Connection::new(),
                Connection::new(),
                Connection::new(),
                Connection::new(),
                Connection::new(),
            ],
        }
    }
}
