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
info: profile set to 'default'
info: setting default host triple to x86_64-unknown-linux-gnu
info: syncing channel updates for 'nightly-x86_64-unknown-linux-gnu'
info: latest update on 2024-01-09, rust version 1.77.0-nightly (ca663b06c 2024-01-08)
info: downloading component 'cargo'
info: downloading component 'clippy'
info: downloading component 'rust-docs'
info: downloading component 'rust-std'
info: downloading component 'rustc'
info: downloading component 'rustfmt'
info: installing component 'cargo'
info: installing component 'clippy'
info: installing component 'rust-docs'
 14.7 MiB /  14.7 MiB (100 %)  14.1 MiB/s in  1s ETA:  0s
info: installing component 'rust-std'
 25.9 MiB /  25.9 MiB (100 %)  22.9 MiB/s in  1s ETA:  0s
info: installing component 'rustc'
 61.6 MiB /  61.6 MiB (100 %)  24.3 MiB/s in  2s ETA:  0s
info: installing component 'rustfmt'
info: default toolchain set to 'nightly-x86_64-unknown-linux-gnu'

  nightly-x86_64-unknown-linux-gnu installed - rustc 1.77.0-nightly (ca663b06c 2024-01-08)


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
