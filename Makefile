run_liquidator:
	cargo run --release -p liquidator run
debug_liquidator:
	cargo run -p liquidator run

run_integration_test:
	cargo run --release -p integration_test run
debug_integration_test:
	cargo run -p integration_test run

generate_contract_bindings:
	cargo run -p contract_bindings run