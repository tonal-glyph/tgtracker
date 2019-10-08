//@ ##tgtracker
//@
//@ - nodes can have instruments, patterns, sequences, recordings, and edges
//@ - graphs can have sequences, devices, channels
//@ - devices can have plugins, sockets, directions
//@ - sequences can have samples, patterns, notes, bars, beats, tracks, and channels
//@ - channels can have directions, plugins, tracks, sockets
//@ - documents can have formulas, statics, dynamics, snippets, graphs
//@
//@ ### document
//@
//@ The UI wraps itself around the loaded document. When there is no document loaded, a helpful message appears explaining the keybindings. A document is a gzipped package of data that represents a 'song.' A document may consist of text buffers (markdown, ChucK code, FAUST code, etc.), images (album covers, images of mathematical formulas), and binary blobs (chugins, FAUST libs, audio files, video clips). It also contains node graph data, pattern data, envelope data, automation data, etc.
//@
//@ ### TODO
//@
//@ - [ ] - Use the flate2 crate?
//@ - [ ] - Specify document format, gzipped file to be played back by tgtracker. Pattern data should be serializable,
