rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run
	
build-container:
# Build the container
	docker build -t proj1 .

sample:
	 docker run --rm -it proj1 plot --filename "demo.png" --caption "sample size vs frequency" --xlabel "sample size" --ylabel "frequency of sample size"

release:
	cargo build --release

all: format lint test run
