## tgtracker_specs

The specs crate uses Tango for literate programming, i.e. embedding the specification code in the specification documentation. When you want to build or test the specs simply use `cargo build` and `cargo test`. Tango will take all the .md files and transfer the rust code blocks to Rust source files, commenting out the markdown content and preserving it.

### Disambiguation

In the demoscene certain tracker music terms are used interchangeably and may confuse newcomers so this section exists in the spec documents to help readers understand why I chose certain terms to use here.

- A 'sample' in trackers is often meant as a sound file loaded and played as notes. A sample in digital signal processing is also a snapshot of a sound recording indicating frequency and amplitude at that exact moment of the recording. At the 44kHz sample rate, 44,000 samples are taken every second. In tgtracker this latter definition is used when we say 'sample.' Hence, a sound recording loaded and played as notes is referred to as a 'phrase'.

- A module or 'mod' is often used to refer to the song or project that is loaded in the tracker. Since tgtracker projects are a little more complex than a 'song', we use the term 'document' to refer to the master project file. This avoids confusion with Rust's module system of code encapsulation. A document must always have at least one channel. A natural limit to the number of channels should be established, but 32/48/64/128 are most desirable.

- A static piece of data in a loaded document could be a text buffer, pattern data, node graphs, envelopes, automation, even images. Though the contents of these data may change, these changes won't normally happen in realtime, hence they are static data. 'Static' in this case does not mean immutable.

- A dynamic is basically a livecoding interface or REPL (Read, Evaluate, Print, Loop) piece of data. Livecoding with ChucK or a Jupyter notebook are 2 possible examples.

- The node graph is a directed acyclic graph or DAG. It is used to chain together and route 'machines' which may generate and/or process audio data, and may be VST plugins, Faust diagrams, or some internal DSP plugin. Each node represents an instrument or effect and each one may have one or more 'sockets' or 'ports' and one or more 'connections' or 'directions' (a.k.a. edges). Nodes may be labeled.

- An instrument is often meant as something that generates audio, be it a 'phrase' (sound recording), VST plugin, or something that connects to a sound generator (like a MIDI instrument.) Instruments can have their volume, panning, effects, and automation controlled individually.

- A pattern is a block of note and automation data that causes audio playback. The sequence is a collection of patterns that are played sequentially. Patterns can be repeated and cloned, and there can be a potentially infinite number of patterns in the sequence. Patterns are 64 notes long by default. The length of patterns can be controlled. A 64-note pattern can be divided into 4 measures of 16 notes each. Patterns can also be saved to disk.

- An effect is a tracker command using the effect column to process the track's channel, things like delay, reverb, flange, phaser, chorus, portamento, vibrato, tremolo, arpeggio

- A plugin is a node type that can host a phrase, VST plugin, dynamic (livecoder), or something that generates/processes sound. They usually come in two flavors, an instrument and a chainable effect.
