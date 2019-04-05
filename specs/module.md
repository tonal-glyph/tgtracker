# everything is a module

## hierarchy

### document

The UI wraps itself around the loaded document. When there is no document loaded, a helpful message appears explaining the keybindings. A document is a gzipped package of data that represents a 'song.' A document may consist of text buffers (markdown, ChucK code, FAUST code, etc.), images (album covers, images of mathematical formulas), and binary blobs (chugins, FAUST libs, audio files, video clips). It also contains node graph data, pattern data, envelope data, automation data, etc.

A document always has at least one channel. A natural limit to the number of channels must be decided, but 32/48/64/128 are most desirable.

Static data examples include text buffers, pattern data, node graphs, envelopes, automation, images

- nodes can have instruments, patterns, sequences, recordings, and edges
- graphs can have sequences, devices, channels
- devices can have plugins, sockets, directions
- sequences can have samples, patterns, notes, bars, beats, tracks, and channels
- channels can have directions, plugins, tracks, sockets
- documents can have formulas, statics, dynamics, snippets, graphs

- bar mod
- beat mod
- note mod
- pattern mod
- track mod
- channel mod
- sample mod
- sequence mod

---

- edge mod
- node mod
- graph mod

---

- instrument mod
- phrase mod
- envelope mod
- point mod

---

- device mod
- plugin mod
- socket mod
- direction mod

---

- document mod
- formula mod
- static mod
- dynamic mod
- snippet mod
