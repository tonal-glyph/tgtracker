#![deny(missing_docs)]
// Copyright (c) 2019 Andrew Prentice
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `tgtracker` boots the main thread and loads various windows
#![allow(unused_imports)]
use std::env;
use std::process::Command;
fn main() {
    println!("Opening alacritty for terminal interface...");
    Command::new("alacritty")
        .spawn()
        .expect("failed to run alacritty")
        .wait()
        .expect("failed to wait for subprocess");
}

#[cfg(test)]
mod tests {
    use std::process::Command;
    // #[test]
    // fn test_ruck() {
    //     Command::new("ruck")
    //         .arg("--help")
    //         .spawn()
    //         .expect("failed to run ruck")
    //         .wait()
    //         .expect("failed to wait for subprocess");
    // }
    #[test]
    fn test_alacritty() {
        Command::new("alacritty")
            .arg("-V")
            .spawn()
            .expect("failed to run alacritty")
            .wait()
            .expect("failed to wait for subprocess");
    }
}
