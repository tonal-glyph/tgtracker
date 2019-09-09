bt:='0'
export RUST_BACKTRACE:=bt
# run clippy
clip:
	@cargo clippy
# run application
run:
	@cargo run
#for each git submodle, pull
subf:
    @git submodule foreach git pull origin master
#init git submodules
subu:
    @git submodule update --init --recursive
#output default rustfmt.toml
form:
	@rustfmt --print-config default rustfmt.toml
#run tests
test: build
	@cargo test --all -- --nocapture
#delete logs
dlog:
	@rm -v *.log
#spam tests, uses figlet/toilet
# @spam:
# 	{ \
# 		figlet test; \
# 		cargo build --color always 2>&1; \
# 		cargo test  --color always -- --color always 2>&1; \
# 	} | less
# only run tests matching PATTERN
filter PATTERN: build
	@cargo test {{PATTERN}}
# test with backtrace
backtrace:
	RUST_BACKTRACE:=1 cargo test
#build project
build:
	@cargo build
#check project
check:
	@cargo check
#watch project
# watch COMMAND='test':
# 	cargo watch --clear --exec {{COMMAND}}
version := `sed -En 's/version[[:space:]]*=[[:space:]]*"([^"]+)"/v\1/p' Cargo.toml`
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
#run tokei
toke:
	@tokei
#check for FIXME/TODO and long lines
@lint:
	echo Checking for FIXME/TODO...
	! grep --color -En 'FIXME|TODO' src/*.rs
	echo Checking for long lines...
	! grep --color -En '.{101}' src/*.rs
#sed search and replace
replace FROM TO:
	find src -name '*.rs' | xargs sed -i '' -E 's/{{FROM}}/{{TO}}/g'
#generate changelog
clog:
	changelog -vP -o CHANGELOG.md
#enter rust repl
repl:
	@irust
#format rust source
rfmt:
    @rustfmt -q --emit files src/**/*.rs
    @echo "Formatted Rust code."
#build for musl
# musl:
#     PKG_CONFIG_ALLOW_CROSS:=1 RUSTFLAGS=-Ctarget-cpu=generic cargo build --target=x86_64-unknown-linux-musl --release
# Local Variables:
# mode: makefile
# End:
# vim: set ft=make :
