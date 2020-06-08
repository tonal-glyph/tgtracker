# tonal glyph tracker

[![Codacy Badge](https://api.codacy.com/project/badge/Grade/2b4d117015214886905691720955eb85)](https://app.codacy.com/app/scalarwaves/tgtracker?utm_source=github.com&utm_medium=referral&utm_content=tonal-glyph/tgtracker&utm_campaign=Badge_Grade_Dashboard)
[![FOSSA Status](https://app.fossa.io/api/projects/git%2Bgithub.com%2Ftonal-glyph%2Ftgtracker.svg?type=shield)](https://app.fossa.io/projects/git%2Bgithub.com%2Ftonal-glyph%2Ftgtracker?ref=badge_shield)

[![Build Status](https://travis-ci.org/tonal-glyph/tgtracker.svg?branch=master)](https://travis-ci.org/tonal-glyph/tgtracker)

IMPORTANT: This project is still in the planning and sandbox phase. Please see Projects and Issues for current progress.

tgtracker will be a digital audio workstation with a tracker-inspired interface. When I originally had the idea, I wanted it to be written in Rust as much as possible. After much research into Rust audio libraries and GUI programming, I've realized that a pure Rust tracker would be very hard to pull off right now with Rust having such a young ecosystem. Ideally Rust could replace C++ in some parts, but frameworks like JUCE are just light years ahead of Rust when it comes to native interfaces and audio routing. Other possible features like VST support, Lua scripting, and Qt widgets also pushed me towards C++. Current sandbox efforts are thus focused on these ends.

## Inspiration & History

I first encountered the demoscene in 1996. I used FastTracker and MadTracker to take apart existing modules and make my own. I have wanted since those years to make a tracker of my own. I am new to systems programming so I need all the help I can get for this project. I'm taking inspiration from several existing trackers/DAWs like FastTracker II, MadTracker 2, BuzzTracker, Cockos Reaper, Renoise, and many others.

## Goals

- Cross-platform
- Embeddable in game engines/music applications
- Low latency
- MIDI and OSC support
- Portable/serializable data
- Unique interface and document format

## Non-goals

- Backwards compatability w/ existing modules (XM, IT, etc.) but import to native format would be nice

## License

All libraries and frameworks used to make tgtracker are used under their respective open source licenses.

[![FOSSA Status](https://app.fossa.io/api/projects/git%2Bgithub.com%2Ftonal-glyph%2Ftgtracker.svg?type=large)](https://app.fossa.io/projects/git%2Bgithub.com%2Ftonal-glyph%2Ftgtracker?ref=badge_large)