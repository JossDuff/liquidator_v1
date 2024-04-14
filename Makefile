run_liquidator_v1:
	cargo run --release -p liquidator_v1 run
debug_liquidator_v1:
	cargo run -p liquidator_v1 run

run_integration_test:
	cargo run --release -p integration_test run
debug_integration_test:
	cargo run -p integration_test run

generate_contract_bindings:
	cargo run -p contract_bindings run