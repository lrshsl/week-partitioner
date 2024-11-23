

local:
	cargo run

run:
	cargo build --target wasm32-unknown-unknown
	cp target/wasm32-unknown-unknown/debug/week-partitioner.wasm output/

server:
	basic-http-server output/ &

kill-server:
	kill basic-http-server

clean:
	rm output/*
	cargo clean

