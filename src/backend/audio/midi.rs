use midir::{Ignore, MidiInput, MidiOutput};
use std::error::Error;
use std::io::{stdin, stdout, Write};
use yansi::Paint;

/// Initialize MIDI ports. Pass `--loop` to the main executable to run endlessly
pub fn init() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", Paint::red(err.description())),
    }
}

/// Run initialization logic
fn run() -> Result<(), Box<Error>> {
    let mut midi_in = MidiInput::new("tgtracker-midi-input")?;
    midi_in.ignore(Ignore::None);
    let midi_out = MidiOutput::new("tgtracker-midi-output")?;
    let mut input = String::new();
    loop {
        println!("{}", Paint::green("Available input ports:"));
        for i in 0..midi_in.port_count() {
            print!("{}: {}", Paint::green(i), midi_in.port_name(i)?);
        }
        println!("{}", Paint::green("\nAvailable output ports:"));
        for i in 0..midi_out.port_count() {
            println!("{}: {}", Paint::green(i), midi_out.port_name(i)?);
        }
        // run in endless loop if "--loop" parameter is specified
        match ::std::env::args().nth(1) {
            Some(ref arg) if arg == "--loop" => {}
            _ => break,
        }
        print!("\nPress <enter> to retry ...");
        stdout().flush()?;
        input.clear();
        stdin().read_line(&mut input)?;
        println!("\n");
    }
    Ok(())
}
