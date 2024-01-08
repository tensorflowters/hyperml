include .env

install_rustup:
	@/bin/bash -c "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"

cargo_version:
	@cargo --version

run_tabby:
	@docker run -it --gpus all -p 8080:8080 -v $HOME/.tabby:/data tabbyml/tabby serve --model TabbyML/StarCoder-1B --device cuda

create_v1:
	@cargo new hyperml_v1

cg_build:
	@cd hyperml_v1 && \
		cargo build

cg_run:
	@cd hyperml_v1 && \
		cargo run

cg_test:
	@cd hyperml_v1 && \
		cargo test

cg_doc:
	@cd hyperml_v1 && \
		cargo doc

cg_help:
	@cargo help