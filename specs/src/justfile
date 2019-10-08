# run clippy
clip:
	@cargo clippy
# run application
run:
	@cargo run
# build specs
build:
	@cargo build
#build and test
test: build
	@cargo test -- --nocapture
# clean ALL artifacts
clean:
	@echo "Removing all build artifacts..."
	@rm -rf **/target
	@echo "Removing all Rust source files..."
	@rm -Rv src/**/*.rs
	@cargo clean -q
# So vim can handle justfile like Makefile syntax
# Local Variables:
# mode: makefile
# End:
# vim: set ft=make :
