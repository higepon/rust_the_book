all: format
	cargo run "How" ./poem.txt

format: 
	rustfmt src/**/*.rs; rustfmt src/*.rs

test: format
	cargo test