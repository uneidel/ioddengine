build:
	cargo build --target wasm32-unknown-unknown --release

#wasmi_cli <WASM_FILE> --invoke <FUNC_NAME> [<FUNC_ARGS>]*
run:
	wasmi_cli ../target/wasm32-unknown-unknown/release/engine.wasm --invoke parse

prepare:
	cargo install wasmi_cli

