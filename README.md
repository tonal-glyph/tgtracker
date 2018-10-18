# tonal glyph tracker
A cross-platform digital audio workstation with a tracker inspired interface

The project is still in its planning phase. I'm starting to get some good feedback. Modern music trackers like Renoise and Reaper offer features like VST/AU/DSSI/LADSPA plug-ins and scripting languages. The tracker style interface remains a powerful and efficient way to compose music despite the concepts age. It goes all the way back to the player piano!

Another powerful way to generate and process sound is a graph-based interface with nodes representing modular instruments and effects that can be chained together. Adding support for music programming languages like **ChucK**, **Csound**, and **Pd** opens up even more possibilities. Through the analog loophole, all of these audio languages and pretty much anything else can be used as nodes in the modular system.

**Conrod** has a graph widget which uses **daggy**, and **dsp-chain** uses daggy for its audio processing and analysis graphs. Overlaying one on top of the other should be easy with a node walker.

**daggy** uses **petgraph** under the hood, same as most other graph structures in crates like **processors** and **pcm-flow**.

## Goals
- Cross-platform at least Windows/Mac/Linux and hopefully webasm/emscripten
- Memory-safe, primarily through Rust's safety features
- Compatible with existing standards
- Embeddable in game engines/music applications
- Portable/serializable file formats

## Nice-to-haves but fraught with issues
- Able to play many module formats through libopenmpt
- VST2/3, DSSI, LADSPA, LV2, AU plugin support
- A scripting API
- Internal synthesis and DSP
- MIDI, Sysex, and OSC support

## History

I first entered the demoscene in 1996 when I was fifteen where I made songs under the names gmork and bitsmart. I used Fasttracker II to take apart existing XM modules and make my own. Several years passed and I moved onto Madtracker 2. This is where VST plugins came into the picture and things got out of control. I have wanted since those years to make a tracker of my own. I am new to systems programming so I need all the help I can get for this project. Rust is still having some growing pains but that hasn't stopped me from being an early adopter of its philosophy. I feel free to write system-level code without worrying about making foolish beginners mistakes.

My design ideas are heavily influenced by the following great projects in no particular order: Fasttracker II, Madtracker 2, the Sonique music player and Night55, specifically Scirocco's music, Buzztracker, Aodix/NoiseTrekker/Psycle/Renoise, SynthEdit, Synthmaker/Flowstone, SuperCollider, PureData, Sonic Pi, ChucK, Reaper and Reason.
