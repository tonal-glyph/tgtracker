extern crate memmap;
extern crate oxdz;
extern crate cpal;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{Write, stdout};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;
use memmap::Mmap;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("usage: {} <filename>", Path::new(&args[0]).file_name().unwrap().to_str().unwrap());
        return;
    }

    match run(args) {
        Ok(_)  => {},
        Err(e) => eprintln!("error: {}", e),
    }
}

fn run(args: Vec<String>) -> Result<(), Box<Error>> {

    // Set up our audio output
    let device = cpal::default_output_device().expect("Failed to get default output device");

    // Create event loop
    let format = cpal::Format{
        channels   : 2,
        sample_rate: cpal::SampleRate(44100),
        data_type  : cpal::SampleFormat::I16,
    };
    let event_loop = cpal::EventLoop::new();
    let stream_id = event_loop.build_output_stream(&device, &format)?;
    event_loop.play_stream(stream_id);


    let info = Arc::new(Mutex::new(oxdz::FrameInfo::new()));

    {
        let info = info.clone();
        thread::spawn(move || {
            let filename = &args[1];
            let file = File::open(filename).unwrap();

            let mut oxdz = {
                let mmap = unsafe { Mmap::map(&file).expect("failed to map the file") };
                oxdz::Oxdz::new(&mmap[..], 44100, "").unwrap()
            };

            // Display basic module information
            let mut mi = oxdz::ModuleInfo::new();
            oxdz.module_info(&mut mi);
            println!("Title : {}", mi.title);
            println!("Format: {}", mi.description);

            event_loop.run(move |_, data| {
                match data {
                    cpal::StreamData::Output{buffer: cpal::UnknownTypeOutputBuffer::I16(mut buffer)} => {
                        {
                            let mut fi = info.lock().unwrap();
                            oxdz.frame_info(&mut fi);
                        }
                        oxdz.fill_buffer(&mut buffer, 0);
                    },

                    _ => { }
                }
            });
        });
    };


    loop {
        {
            let fi = info.lock().unwrap();
            print!("pos:{:3} - row:{:3} \r", fi.pos, fi.row);
        }
        stdout().flush().unwrap();
        std::thread::sleep(Duration::from_millis(50));
    }
}
