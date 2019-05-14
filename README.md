# tonal glyph tracker

[![Codacy Badge](https://api.codacy.com/project/badge/Grade/2b4d117015214886905691720955eb85)](https://app.codacy.com/app/scalarwaves/tgtracker?utm_source=github.com&utm_medium=referral&utm_content=tonal-glyph/tgtracker&utm_campaign=Badge_Grade_Dashboard)
[![Build Status](https://travis-ci.org/tonal-glyph/tgtracker.svg?branch=master)](https://travis-ci.org/tonal-glyph/tgtracker)

IMPORTANT: This project is still in the planning and sandbox phase. Please see Projects and Issues for current progress.

tgtracker will be a digital audio workstation with a tracker-inspired interface. It will be written primarily in Rust. The [nannou](https://github.com/nannou-org/nannou) creative coding framework has colinear goals with tgtracker and reexports crates I plan to use anyway, so for now it's part of the family.

## Goals

- Cross-platform
- Low latency
- Embeddable in game engines/music applications
- Portable/serializable data
- Use new technologies like Wayland, Vulkan, and webassembly
- Unique interface and document format
- MIDI and OSC support

## Non-goals

- Backwards compatability w/ existing modules (XM, IT, etc.) but import to native format would be nice

## History

I first entered the demoscene in 1996. I used various trackers to take apart existing modules and make my own. I have wanted since those years to make a tracker of my own. I am new to systems programming so I need all the help I can get for this project.

