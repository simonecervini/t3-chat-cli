# T3.chat CLI

**Love T3.chat but tired of @t3dotgg focusing only on the web version when you live in the terminal?**

Here's your solution: the T3.chat CLI! Built with Rust for maximum performance and blazing speed, this tool brings the power of T3.chat directly to your command line.

## Features

- 🦀 Written in Rust
- ⚡ Blazingly fast
- 💾 Memory safe
- 🔥 Web-scale performance
- ⚙️ Enterprise-ready

## Installation

```bash
cargo install --git https://github.com/simonecervini/t3-chat-cli
```

### Setting up PATH

After installation, you might need to add Cargo's bin directory to your PATH:

**For Zsh (macOS default):**

```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

**For Bash:**

```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

## Usage

```bash
t3-chat-cli
```

## Development

### Building from source

```bash
# Clone the repository
git clone https://github.com/simonecervini/t3-chat-cli
cd t3-chat-cli

# Run directly
cargo run

# Or build for release
cargo build --release
./target/release/t3-chat-cli
```

_Made with ❤️ by Simone Cervini_
