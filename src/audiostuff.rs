use std::thread;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

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

pub fn cpal_stuff() {
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
            cpal::SampleFormat::F32 => run::<f32>(&device, &config.into()),
            cpal::SampleFormat::I16 => run::<i16>(&device, &config.into()),
            cpal::SampleFormat::U16 => run::<u16>(&device, &config.into()),
        };
    }));
}

fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig)
where
    T: cpal::Sample,
{
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let mut saw: Saw = Saw {
        frequency: 110.0,
        count: 0,
        val: 0.0,
    };

    let stream = device
        .build_output_stream(
            config,
            move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                for frame in data.chunks_mut(channels) {
                    let value: T = cpal::Sample::from::<f32>(&(saw.next_sample(44_100.0)));
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
