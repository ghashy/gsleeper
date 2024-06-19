// This example shows how to use the GstPlay API.
// The GstPlay API is a convenience API to allow implement playback applications
// without having to write too much code.
// Most of the tasks a play needs to support (such as seeking and switching
// audio / subtitle streams or changing the volume) are all supported by simple
// one-line function calls on the GstPlay.

use std::{env, f64::consts::PI, path::PathBuf, time::Duration};

use anyhow::Error;
use clap::Parser;
use duration_str::parse;
use system_shutdown::shutdown;

use gstreamer as gst;
use gstreamer_play::{Play, PlayMessage, PlaySignalAdapter, PlayVideoRenderer};

mod duration_parse;
mod run;

fn main_loop(uri: &str, duration: Duration) -> Result<(), Error> {
    gst::init()?;

    let play = Play::new(None::<PlayVideoRenderer>);
    play.set_uri(Some(uri));
    play.play();
    play.set_volume(1.0);

    let start_time = std::time::Instant::now();
    let p = play.clone();
    std::thread::spawn(move || loop {
        let elapsed = std::time::Instant::now().duration_since(start_time);
        // start + (end - start) * scalar
        let progress = elapsed.as_secs_f64() / duration.as_secs_f64();
        let new_volume = f64::clamp(volume_curve(1.0 - progress), 0.0, 1.0);
        p.set_volume(new_volume);
        println!("Volume changed: {}", new_volume);

        std::thread::sleep(Duration::from_secs(1));
    });

    let mut result = Ok(());
    for msg in play.message_bus().iter_timed(gst::ClockTime::NONE) {
        match PlayMessage::parse(&msg) {
            Ok(PlayMessage::EndOfStream) => {
                play.stop();
                break;
            }
            Ok(PlayMessage::Error { error, details }) => {
                result = Err(error);
                play.stop();
                break;
            }
            Ok(PlayMessage::VolumeChanged { volume }) => {
                if volume == 0.0 {
                    break;
                }
            }
            Ok(_) => (),
            Err(_) => unreachable!(),
        }
    }

    // Set the message bus to flushing to ensure that all pending messages are dropped and there
    // are no further references to the play instance.
    play.message_bus().set_flushing(true);

    result.map_err(|e| e.into())
}

/// Sleep music fade app
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to audio/video file
    file: PathBuf,

    /// Sets the time interval (e.g., 1h, 30min, 60sec, 1h25m)
    #[arg(short, long, default_value = "1h", value_parser = parse_duration)]
    time: Duration,

    /// Run shutdown at end
    #[arg(short, long, default_value_t = false)]
    shutdown: bool,
}

fn example_main() {
    let args = Args::parse();

    let path = args
        .file
        .to_str()
        .expect("Path contains not valid unicode characters.");
    let uri = format!("file://{}", path);

    match main_loop(&uri, args.time) {
        Ok(r) => r,
        Err(e) => eprintln!("Error! {e}"),
    }
    if args.shutdown {
        match shutdown() {
            Ok(_) => println!("Shutting down, bye!"),
            Err(error) => eprintln!("Failed to shut down: {}", error),
        }
    }
}

fn main() {
    run::run(example_main);
}

fn volume_curve(t: f64) -> f64 {
    t.powf(3.)
}

fn parse_duration(s: &str) -> Result<Duration, String> {
    parse(s)
}
