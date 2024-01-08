# HyperML documentation

## Requirements

### WSL2

This project was built under WSL2. Check here: <https://learn.microsoft.com/fr-fr/windows/wsl/install>

### Install rustup

You need to have curl installed.

Run the following in your terminal, then follow the onscreen instructions to install Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

If you prefer I wrote a makefile with the exacts commands I used.

So just run :

```bash
make install_rustup
```

Output :

```bash
info: downloading installer

Welcome to Rust!

This will download and install the official compiler for the Rust
programming language, and its package manager, Cargo.

Rustup metadata and toolchains will be installed into the Rustup
home directory, located at:

  /home/athernatos/.rustup

This can be modified with the RUSTUP_HOME environment variable.

The Cargo home directory is located at:

  /home/athernatos/.cargo

This can be modified with the CARGO_HOME environment variable.

The cargo, rustc, rustup and other commands will be added to
Cargo's bin directory, located at:

  /home/athernatos/.cargo/bin

This path will then be added to your PATH environment variable by
modifying the profile files located at:

  /home/athernatos/.profile
  /home/athernatos/.bashrc
  /home/athernatos/.zshenv

You can uninstall at any time with rustup self uninstall and
these changes will be reverted.

Current installation options:


   default host triple: x86_64-unknown-linux-gnu
     default toolchain: stable (default)
               profile: default
  modify PATH variable: yes

1) Proceed with installation (default)
2) Customize installation
3) Cancel installation
>1

info: profile set to 'default'
info: default host triple is x86_64-unknown-linux-gnu
info: syncing channel updates for 'stable-x86_64-unknown-linux-gnu'
info: latest update on 2023-12-28, rust version 1.75.0 (82e1608df 2023-12-21)
info: downloading component 'cargo'
info: downloading component 'clippy'
info: downloading component 'rust-docs'
info: downloading component 'rust-std'
info: downloading component 'rustc'
info: downloading component 'rustfmt'
info: installing component 'cargo'
info: installing component 'clippy'
info: installing component 'rust-docs'
 14.3 MiB /  14.3 MiB (100 %)  13.1 MiB/s in  1s ETA:  0s
info: installing component 'rust-std'
 23.6 MiB /  23.6 MiB (100 %)  22.1 MiB/s in  1s ETA:  0s
info: installing component 'rustc'
 61.4 MiB /  61.4 MiB (100 %)  24.0 MiB/s in  2s ETA:  0s
info: installing component 'rustfmt'
info: default toolchain set to 'stable-x86_64-unknown-linux-gnu'

  stable-x86_64-unknown-linux-gnu installed - rustc 1.75.0 (82e1608df 2023-12-21)


Rust is installed now. Great!

To get started you may need to restart your current shell.
This would reload your PATH environment variable to include
Cargo's bin directory ($HOME/.cargo/bin).

To configure your current shell, run:
source "$HOME/.cargo/env"
```

### ZSH

Check here: <https://ohmyz.sh/>

### Tabby

VSCode code completion working in Tabby.

You need to have Docker Desktop and VSCode installed.

Check here: <https://tabby.tabbyml.com/docs/installation/docker>
