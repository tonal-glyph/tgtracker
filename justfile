bt='0'

export RUST_BACKTRACE=bt

clip:
	cargo clippy

run:
	cargo run

subf:
    git submodule foreach git pull origin master

subu:
    git submodule update --init --recursive

form:
	rustfmt --print-config default rustfmt.toml

test: build
	cargo test

@spam:
	{ \
		figlet test; \
		cargo build --color always 2>&1; \
		cargo test  --color always -- --color always 2>&1; \
	} | less

# only run tests matching PATTERN
filter PATTERN: build
	cargo test {{PATTERN}}

# test with backtrace
backtrace:
	RUST_BACKTRACE=1 cargo test

build:
	cargo build

check:
	cargo check

watch COMMAND='test':
	cargo watch --clear --exec {{COMMAND}}

version = `sed -En 's/version[[:space:]]*=[[:space:]]*"([^"]+)"/v\1/p' Cargo.toml`

# publish to crates.io
# publish: lint clippy test
# 	git branch | grep '* master'
# 	git diff --no-ext-diff --quiet --exit-code
# 	cargo publish
# 	git tag -a {{version}} -m 'Release {{version}}'
# 	git push github {{version}}

# clean up feature branch BRANCH
done BRANCH:
	git checkout {{BRANCH}}
	git pull --rebase github master
	git checkout master
	git pull --rebase github master
	git branch -d {{BRANCH}}

# install just from crates.io
install:
	cargo install -f just

# install development dependencies
install-dev-deps:
	rustup install nightly
	rustup update nightly
	rustup run nightly cargo install -f clippy
	cargo install -f cargo-watch
	cargo install -f cargo-check

# count non-empty lines of code
sloc:
	@cat src/*.rs | sed '/^\s*$/d' | wc -l

toke:
	tokei

@lint:
	echo Checking for FIXME/TODO...
	! grep --color -En 'FIXME|TODO' src/*.rs
	echo Checking for long lines...
	! grep --color -En '.{101}' src/*.rs

replace FROM TO:
	find src -name '*.rs' | xargs sed -i '' -E 's/{{FROM}}/{{TO}}/g'

clog:
	changelog -vP -o CHANGELOG.md

# Local Variables:
# mode: makefile
# End:
# vim: set ft=make :
