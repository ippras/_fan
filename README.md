# FAN

The Fatty acid names project

* [aocs.org 1](https://www.aocs.org/resource/trivial-names-of-fatty-acids-part-1)
* [aocs.org 2](https://www.aocs.org/resource/trivial-names-of-fatty-acids-part-2)
* [byrdwell.com](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)

## Run

`trunk serve --address=0.0.0.0`

**nix**:

`RUST_LOG=none,fan=trace cargo run`

**win**:

`$env:RUST_LOG="none,fan=trace"` `cargo run`

## Build

`rustup target add wasm32-unknown-unknown`

`trunk build --release --public-url fan`

## CRLF to LF

After changing your Git settings, say by running:

`git config --global core.autocrlf input`

You should refresh the indexes with

`git rm --cached -r .`

And rewrite the Git index with

`git reset --hard`