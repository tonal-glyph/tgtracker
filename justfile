bt:='0'
export RUST_BACKTRACE:=bt
# test with backtrace
backtrace:
	RUST_BACKTRACE=1 cargo test
#build project
build:
	@cargo build
#generate changelog
clog:
	changelog -vP -o CHANGELOG.md
#check project
check:
	@cargo check
# run clippy
clip:
	@cargo clippy
#delete logs
dlog:
	@rm -v *.log
# clean up feature branch BRANCH
done BRANCH:
	git checkout {{BRANCH}}
	git pull --rebase github master
	git checkout master
	git pull --rebase github master
	git branch -d {{BRANCH}}
# only run tests matching PATTERN
filter PATTERN: build
	@cargo test {{PATTERN}}
#output default rustfmt.toml
form:
	@rustfmt --print-config default rustfmt.toml
# install some rust tools
install:
	cargo install -f alacritty just ruck
# install development dependencies
install-dev-deps:
	rustup install nightly
	rustup update nightly
	rustup run nightly cargo install -f clippy
	cargo install -f cargo-watch
	cargo install -f cargo-check
#kill running chuck vms
killck:
	@sudo killall chuck
#check for FIXME/TODO and long lines
@lint:
	echo Checking for FIXME/TODO...
	! grep --color -En 'FIXME|TODO' src/*.rs
	echo Checking for long lines...
	! grep --color -En '.{101}' src/*.rs
#build for musl
# musl:
#     PKG_CONFIG_ALLOW_CROSS:=1 RUSTFLAGS=-Ctarget-cpu=generic cargo build --target=x86_64-unknown-linux-musl --release
# publish to crates.io
# publish: lint clippy test
# 	git branch | grep '* master'
# 	git diff --no-ext-diff --quiet --exit-code
# 	cargo publish
# 	git tag -a {{version}} -m 'Release {{version}}'
# 	git push github {{version}}
#sed search and replace
replace FROM TO:
	find src -name '*.rs' | xargs sed -i '' -E 's/{{FROM}}/{{TO}}/g'
#enter rust repl
repl:
	@irust
#format rust source
rfmt:
    @rustfmt -q --emit files **/*.rs
    @echo "Formatted Rust code."
# run application
run:
	@cargo run
# count non-empty lines of code
sloc:
	@cat src/*.rs | sed '/^\s*$/d' | wc -l
#spam tests, uses figlet/toilet
# @spam:
# 	{ \
# 		figlet test; \
# 		cargo build --color always 2>&1; \
# 		cargo test  --color always -- --color always 2>&1; \
# 	} | less
#for each git submodle, pull
subf:
    @git submodule foreach git pull origin master
#init git submodules
subu:
    @git submodule update --init --recursive
#run tests
test: build
	@cargo test --all -- --nocapture
#run tokei
toke:
	@tokei
#watch project
# watch COMMAND='test':
# 	cargo watch --clear --exec {{COMMAND}}
# version := `sed -En 's/version[[:space:]]*=[[:space:]]*"([^"]+)"/v\1/p' Cargo.toml`
# Local Variables:
# mode: makefile
# End:
# vim: set ft=make :
