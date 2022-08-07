all: format
	cargo run "arg1" ./poem.txt

format: 
	rustfmt src/**/*.rs; rustfmt src/*.rs

test: format
	cargo test