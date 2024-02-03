
## Install kvm on host to boot the project
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

install_rustup:
	@curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

uninstall_rustup:
	@rustup self uninstall

start_minimal_kernel:
	@cargo build
	@cargo install bootimage
	@cargo bootimage
	@cargo run

test_kernel:
	@cargo test