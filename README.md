<h3 align="center">
    <img src="https://raw.githubusercontent.com/sammhansen/fuzzel-rbw/unstable/.assets/logos/bitwarden.svg" width="100" alt="Logo"/><br/>
    Fuzzel RBW
</h3>

<p align="center">
    <a href="https://github.com/sammhansen/fuzzel-rbw/stargazers"><img src="https://img.shields.io/github/stars/sammhansen/fuzzel-rbw?colorA=363a4f&colorB=b7bdf8&style=for-the-badge"></a>
    <a href="https://github.com/sammhansen/fuzzel-rbw/issues"><img src="https://img.shields.io/github/issues/sammhansen/fuzzel-rbw?colorA=363a4f&colorB=f5a97f&style=for-the-badge"></a>
    <a href="https://github.com/sammhansen/fuzzel-rbw/contributors"><img src="https://img.shields.io/github/contributors/sammhansen/fuzzel-rbw?colorA=363a4f&colorB=a6da95&style=for-the-badge"></a>
</p>

<p align="center">
    🦀 frbw —  a minimal and fast Rust-powered tool that brings the power of Bitwarden to your Wayland desktop using <a href="https://github.com/doy/rbw">RBW</a> and <a href="https://codeberg.org/dnkl/fuzzel">Fuzzel</a>.  With a single keybind, you can search, select, and type credentials from your secure vault — all without leaving the keyboard.
</p>

# Installation
## Build from source
### Prequisites
- You need rust - <a href="https://www.rust-lang.org/tools/install">install rust</a>

  
- Clone this repo
  ```
  git clone https://github.com/sammhansen/fuzzel-rbw.git
  ```
- Build
  ```
  cargo build --release
  ```
- Copy the binary to `/usr/bin/`
  ```
  sudo cp target/release/frbw /usr/bin/
  ```
  
