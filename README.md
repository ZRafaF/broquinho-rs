<!--
 Copyright 2023 rafae

 Licensed under the Apache License, Version 2.0 (the "License");
 you may not use this file except in compliance with the License.
 You may obtain a copy of the License at

     http://www.apache.org/licenses/LICENSE-2.0

 Unless required by applicable law or agreed to in writing, software
 distributed under the License is distributed on an "AS IS" BASIS,
 WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 See the License for the specific language governing permissions and
 limitations under the License.
-->

# Broquinho

Breakout like game made with Rust and Macroquad. Developed as a way to start learning Rust.

Made using the crate [macroquad](https://github.com/not-fl3/macroquad/)

## How to run

### Development

1. Run with `cargo run` this will build an app and the wasm files.

### WASM

1. Run `cargo build --target wasm32-unknown-unknown` to build the `.wasm`.

   > If you want you can run `cargo build --target wasm32-unknown-unknown --release` to create a release build.

2. If you want to run it on the web run `basic-http-server ./build/` the second argument is the path to the dir with the `index.html` file on.
   > The `index.html` links to the wasm file, check [macroquad wasm](https://github.com/not-fl3/macroquad/#wasm) instructions for more details.
