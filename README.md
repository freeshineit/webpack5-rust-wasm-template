# webpack5-rust-wasm-template

![build](https://github.com/freeshineit/webpack5-rust-wasm-template/workflows/build/badge.svg)

## Use

```bash
# install dependency
yarn install

# development
# 如果运行时 rust没有编译 ， 请移除node_modules重新安装
# 支持热更新
yarn run dev

# production
yarn run build

# https://github.com/http-party/http-server
# version >= 18
cd dist && http-server -p 8080 .
```

## Start

### webpack

```bash
yarn add webpack webpack-cli webpack-dev-server copy-webpack-plugin -D
```

### typescript

```bash
yarn add @swc/core swc-loader typescript -D
```

### sass

```bash
yarn add sass css-loader style-loader sass-loader -D

```

[webpack config](./webpack.config.js)

### wasm-pack

rust 编译成 `WebAssembly` 需要 [wasm-pack](https://rustwasm.github.io/wasm-pack/)

```bash
# 不建议全局安装
# 如果wasm-pack 有问题， 请删除 `node_modules`重新安装依赖
yarn add wasm-pack -D

# wasm-pack build --target nodejs
# wasm-pack build --target browser

# webpack plugin
yarn add @wasm-tool/wasm-pack-plugin -D

# 创建 rust 项目
cargo new wasm --lib
```

在[wasm](./wasm)中的[Cargo.toml](./wasm/Cargo.toml)中添加下面依赖

```toml
[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wasm-bindgen = "0.2"

# `wee_alloc` is a tiny allocator for wasm that is only ~1K in code size
# compared to the default allocator's ~10K. It is slower than the default
# allocator, however.
wee_alloc = { version = "0.4.5", optional = true }

[features]
default = ["wee_alloc"]

[dev-dependencies]
wasm-bindgen-test = "0.3"

[package.metadata.wasm-pack.profile.release]
wasm-opt = ["-Oz", "--enable-mutable-globals"]
```

更新[wasm/src/lib.rs](./wasm/src/lib.rs)

```rust
// 这个很重要， 预加载wasm_bindgen
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[wasm_bindgen]属性 是把当前函数暴露给js使用
#[wasm_bindgen]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}
```

### WebAssembly

使用 rust 编译后的产物`wasm/pkg`, 在[app/index.ts](./app/index.ts)中引入。

```ts
// https://developer.mozilla.org/zh-CN/docs/WebAssembly/Loading_and_running
import('../wasm/pkg').then((module) => {
  // module.add 就是 .wasm 暴露出来的函数
  const add = module.add;
  console.log(add(1, 2));
});
```

## Github action

[.github](./.github/workflows/cl.yml)
