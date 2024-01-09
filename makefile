include .env

install_rustup:
	@/bin/bash -c "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"

install_qemu:
	@egrep -c '(vmx|svm)' /proc/cpuinfo
	@grep -E --color '(vmx|svm)' /proc/cpuinfo
	@if [ $$? -eq 0 ]; then sudo apt install \
		qemu-kvm \
		libvirt-clients \
		libvirt-daemon-system \
		bridge-utils \
		virtinst \
		libvirt-daemon; fi

start_kvm:
	@sudo systemctl enable --now libvirtd

install_gdb:
	@sudo apt install -y gdb

cargo_version:
	@cargo --version

run_tabby:
	@docker run -it --gpus all -p 8080:8080 -v $HOME/.tabby:/data tabbyml/tabby serve --model TabbyML/StarCoder-1B --device cuda

create_v1_cross_comp:
	@cd hyperml_v1 && \
		rustup target add x86_64-unknown-none

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
cg_clean:
	@cd hyperml_v1 && \
		cargo clean

cg_build_x86:
	@cd hyperml_v1 && \
		cargo build --target x86_64-unknown-none
start_debug_x86:
	@gdb target/x86_64-unknown-none/debug/hyperml_v1
# Once GDB is running, connect to the QEMU server by executing the following command in the GDB interface: target remote localhost:1234

