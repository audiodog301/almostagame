use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::thread;

mod audiostuff;
use audiostuff::Saw;

use macroquad::prelude::*;

#[macroquad::main("almostagame")]
async fn main() {
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
    	};}));

    loop {
        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        draw_text("HELLO", 20.0, 20.0, 20.0, DARKGRAY);

        next_frame().await
    }
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
        val: 0.0
    };

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            write_data(data, channels, &mut saw)
        },
        err_fn,
    );
    stream.unwrap().play();

    loop {}
}

fn write_data<T>(output: &mut [T], channels: usize, saw: &mut Saw)
where
    T: cpal::Sample,
{
    for frame in output.chunks_mut(channels) {
        let value: T = cpal::Sample::from::<f32>(&(saw.next_sample(44_100.0)));
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}