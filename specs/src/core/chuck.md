- ruckus - A ChucK creative coding toolkit for Rust
  - [x] ruck - passes system command flag and all other arguments to `chuck` binary
    - [x] Call Rust from ChucK
    - [x] Call ChucK from Rust
  - [x] Test faust2ck, it generates chugins from FAUST code
  - [x] Test Faust ugen, it processes inline FAUST code

```rust
//! ChucK functions
#![allow(
    dead_code,
    unused_imports,
)]
use std::path::Path;
use std::process::{Command, Stdio};
use libc::c_uint;
// use regex::Regex;
// use lazy_static::*;

pub enum ChucKOps {
    Add,
    Remove,
    Replace,
    Status,
}
/// ChuckOps implementation
impl ChucKOps {
    pub const VARIANTS: [ChucKOps; 4] = [
        ChucKOps::Add,
        ChucKOps::Remove,
        ChucKOps::Replace,
        ChucKOps::Status,
    ];
}

/// The form of a ChucK operation
pub struct ChucK {
    pub command: Command,
    pub args: Vec<String>,
    pub shred_id: u32,
    pub pid: u32,
    pub backend: String,
}

/// Runs ChucK in a background loop
fn loop_chuck(p: &Path) {
  Command::new("chuck")
    .arg("--port=8889")
    .arg("--loop")
    .arg("--silent")
    .arg(&p)
    .spawn()
    .expect("failed to run chuck")
    .wait()
    .expect("failed to wait for subprocess");
}

/// Adds a ChucK shred to the VM
/// Takes a Path argument ("")
fn add_shred(p: &Path) {
    Command::new("chuck")
        .arg("+")
        .arg(&p)
        .spawn()
        .expect("failed to run chuck")
        .wait()
        .expect("failed to wait for subprocess");
}


/// Tells the VM to halt and exit if there are no more shreds
fn halt_chuck() {
    Command::new("chuck")
        .arg("--halt")
        .spawn()
        .expect("failed to run chuck")
        .wait()
        .expect("failed to wait for subprocess");
}

/// Kill the ChucK VM
fn kill_chuck() {
    Command::new("chuck")
        .arg("--kill")
        .spawn()
        .expect("failed to run chuck")
        .wait()
        .expect("failed to wait for subprocess");
}

/// Dump ChucK AST to output
fn dump_chuck() {
    Command::new("chuck")
        .arg("--dump")
        .arg("scripts/long.ck")
        .spawn()
        .expect("failed to run chuck")
        .wait()
        .expect("failed to wait for the subprocess");
}


// #[cfg(test)]
// #[test]
// fn test_loop_chuck() {
//     loop_chuck(Path::new("/home/dark/proj/tgtracker/specs/src/scripts/long.ck"));
//     halt_chuck();
//     kill_chuck();
// }

// #[test]
// fn test_add_shred() {
//     loop_chuck(Path::new("/home/dark/proj/tgtracker/specs/src/scripts/long.ck"));
//     add_shred(Path::new("/home/dark/proj/tgtracker/specs/src/scripts/long.ck"));
//     halt_chuck();
//     kill_chuck();
// }

// #[test]
// fn test_kill_chuck() {
//     loop_chuck(Path::new("/home/dark/proj/tgtracker/specs/src/scripts/long.ck"));
//     halt_chuck();
//     kill_chuck();
// }

// #[test]
// fn test_halt_chuck() {
//     loop_chuck(Path::new("/home/dark/proj/tgtracker/specs/src/scripts/long.ck"));
//     halt_chuck();
//     kill_chuck();
// }

```
