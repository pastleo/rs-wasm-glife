# 🦀🕸️ `wasm-pack-template`

A template for kick starting a Rust and WebAssembly project using
[`wasm-pack`](https://github.com/rustwasm/wasm-pack).

This template is designed for compiling Rust libraries into WebAssembly and
publishing the resulting package to NPM.

* Want to use the published NPM package in a Website? [Check out
  `create-wasm-app`.](https://github.com/rustwasm/create-wasm-app)
* Want to make a monorepo-style Website without publishing to NPM? Check out
  [`rust-webpack-template`](https://github.com/rustwasm/rust-webpack-template)
  and/or
  [`rust-parcel-template`](https://github.com/rustwasm/rust-parcel-template).

## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.

## 🚴 Usage

### 🐑 Use `cargo generate` to Clone this Template

[Learn more about `cargo generate` here.](https://github.com/ashleygwilliams/cargo-generate)

```
cargo generate --git https://github.com/rustwasm/wasm-pack-template.git --name my-project
cd my-project
```

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### 🎁 Publish to NPM with `wasm-pack publish`

```
wasm-pack publish
```

## notes

### install rust and nodejs

```shell
rustup toolchain install stable
rustup default stable
cargo install cargo-generate
cargo install wasm-pack
asdf install nodejs 11.6.0
asdf global nodejs 11.6.0
```

### init project

```shell
cargo generate --git https://github.com/rustwasm/wasm-pack-template -n wasm-glife
cd wasm-glife
mkdir www
cd www
npm install -g create-wasm-app
asdf reshim
npm init wasm-app
npm install
cd ..
```

### start dev

#### to build rust -> native and run

```shell
cargo build && cargo run
# or
./bin/build-run
```

#### to build rust -> wasm

```shell
rustup run stable wasm-pack build -d www/wasm
# or
./bin/wasm-pack
```

#### to run webpack dev server

```shell
cd www
vim index.js # change import path
npm run start
```
