# Serenity + Poise bot template

This is a simple template for make a Discord Bot with Slash Commands using [Serenity](https://github.com/serenity-rs/serenity) and [Poise](https://github.com/serenity-rs/poise).


## Setup (with [nix flakes](https://nixos.wiki/wiki/Flakes))
### Requirements
- [Nix](https://github.com/NixOS/nix)
- [Direnv](https://direnv.net/)

### Steps
- Create a `.env` following [`.env.example`](./.env.example)
- Run `direnv allow`
- Ready to go!


## Setup (without nix flakes)
### Requirements
- [Rust](https://www.rust-lang.org/) >= 1.75
- [Cargo](https://github.com/rust-lang/cargo)

### Steps
- Create a `.env` following [`.env.example`](./.env.example)
- Add `dotenvy` to dependencies
```bash
cargo add dotenvy
```
or modify `Cargo.toml`
```toml
[dependencies]
dotenvy = "0.15"
```
- Use `dotenvy` in [`main.rs`](./src/main.rs)
```rust
#[tokio::main]
pub async fn main() {
    dotenvy::dotenv().unwrap();
    //...
}
```
- Ready to go!