## tgtracker_specs

The specs crate uses Tango for literate programming, i.e. embedding the specification code in the specification documentation. When you want to build or test the specs simply use `cargo build` and `cargo test`. Tango will take all the .md files and transfer the rust code blocks to Rust source files, commenting out the markdown content and preserving it.

