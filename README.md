# textractor-unicode-normalize

## Description
A [Textractor](https://github.com/Artikash/Textractor) extension which applies NFKC unicode normalization to text (e.g will convert the character ﻳ -> ي). Tested in Wine.

## Build
The DLLs can be built on Linux with the following commands
```sh
# Build dependencies
sudo apt install mingw-w64      # Debian/Ubuntu
sudo dnf install mingw64-gcc    # Fedora
sudo pacman -S mingw-w64        # Arch

rustup target add i686-pc-windows-gnu
rustup target add x86_64-pc-windows-gnu

# Building
cargo build --release --target i686-pc-windows-gnu      # x86
cargo build --release --target x86_64-pc-windows-gnu    # x64
```

## Install
Copy the x86/x64 DLLs to the respective Textractor directories, then add them in the Extensions settings on Textractor.
