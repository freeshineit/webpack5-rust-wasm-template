# webpack5-rust-wasm-template

![build](https://github.com/freeshineit/webpack5-rust-wasm-template/workflows/build/badge.svg)

一个现代化的 Webpack5 + Rust + WebAssembly 项目模板，结合了前端构建工具和高性能 WebAssembly 计算能力。

## 📋 目录

- [架构设计](#架构设计)
- [设计思想](#设计思想)
- [快速开始](#快速开始)
- [技术栈](#技术栈)
- [项目结构](#项目结构)

## 🏗️ 架构设计

### 整体架构

```
┌─────────────────────────────────────────────────────────────┐
│                     Browser Environment                      │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌──────────────────────────────────────────────────────┐   │
│  │           TypeScript Application Layer                │   │
│  │  ┌────────────────────────────────────────────────┐  │   │
│  │  │   • UI Logic (index.ts)                        │  │   │
│  │  │   • Event Handling                             │  │   │
│  │  │   • DOM Manipulation                           │  │   │
│  │  └────────────────────────────────────────────────┘  │   │
│  └──────────────────────┬───────────────────────────────┘   │
│                         │                                     │
│                         ▼                                     │
│  ┌──────────────────────────────────────────────────────┐   │
│  │          WASM Bridge & Interface Layer                │   │
│  │  ┌────────────────────────────────────────────────┐  │   │
│  │  │   • wasm-bindgen Generated Code                │  │   │
│  │  │   • Type Definitions (index.d.ts)              │  │   │
│  │  │   • JS/WASM Interop                            │  │   │
│  │  └────────────────────────────────────────────────┘  │   │
│  └──────────────────────┬───────────────────────────────┘   │
│                         │                                     │
│                         ▼                                     │
│  ┌──────────────────────────────────────────────────────┐   │
│  │            WebAssembly Runtime Layer                  │   │
│  │  ┌────────────────────────────────────────────────┐  │   │
│  │  │   • Rust Compiled Code (index_bg.wasm)         │  │   │
│  │  │   • High-Performance Computing                 │  │   │
│  │  │   • Memory Management                          │  │   │
│  │  └────────────────────────────────────────────────┘  │   │
│  └──────────────────────────────────────────────────────┘   │
│                                                               │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│                    Build-Time Architecture                    │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  TypeScript/SCSS     Rust Source                             │
│       │                  │                                    │
│       ▼                  ▼                                    │
│   SWC Loader       wasm-pack                                 │
│       │                  │                                    │
│       ▼                  ▼                                    │
│   JavaScript      WASM + JS Glue                             │
│       │                  │                                    │
│       └────────┬─────────┘                                   │
│                ▼                                              │
│           Webpack 5                                           │
│                │                                              │
│                ▼                                              │
│         Bundle Output                                         │
│    (dist/index.js + .wasm)                                   │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

### 核心组件

#### 1. **TypeScript 应用层** (`app/`)

- **职责**：用户界面逻辑、DOM 操作、事件处理
- **技术**：TypeScript + SCSS
- **特点**：
  - 类型安全的 WASM 模块调用
  - 完整的错误处理机制
  - 模块化的代码组织

#### 2. **Rust 计算层** (`rust/`)

- **职责**：高性能计算、业务逻辑、算法实现
- **技术**：Rust + wasm-bindgen
- **特点**：
  - 内存安全保证
  - 零成本抽象
  - 编译时优化

#### 3. **WASM 桥接层** (`rust/pkg/`)

- **职责**：JS/WASM 互操作、类型转换、内存管理
- **技术**：wasm-bindgen 生成
- **特点**：
  - 自动生成类型定义
  - 高效的数据传输
  - 透明的内存管理

#### 4. **构建工具链** (`webpack.config.js`)

- **职责**：代码编译、打包、优化
- **技术**：Webpack5 + SWC + wasm-pack
- **特点**：
  - 支持热更新
  - 自动 WASM 编译
  - 代码分割优化

## 💡 设计思想

### 1. **分离计算密集型逻辑**

将性能关键的计算逻辑迁移到 Rust/WASM，保留 UI 逻辑在 JavaScript：

```typescript
// ❌ 传统方式：纯 JS 实现（慢）
function heavyComputation(data) {
  // 复杂计算...
}

// ✅ 优化方式：WASM 加速（快）
const result = await ArithmeticOperation.complexCalculation(data);
```

**优势**：

- 🚀 **性能**：接近原生代码速度（通常比 JS 快 10-100 倍）
- 🔒 **安全**：Rust 的内存安全保证
- ⚡ **并发**：充分利用多核 CPU

### 2. **类型安全的接口设计**

通过 TypeScript 类型定义和 Rust 类型系统，实现端到端的类型安全：

```rust
// Rust 端：强类型 + 错误处理
#[wasm_bindgen]
pub fn addition(left: i32, right: i32) -> Result<i32, JsValue> {
    Self::addition_internal(left, right)
        .map_err(|_| JsValue::from_str("Overflow in addition"))
}
```

```typescript
// TypeScript 端：类型推导 + 错误捕获
try {
  const result = ArithmeticOperation.addition(1, 2); // 类型安全
} catch (error) {
  console.error('Calculation failed:', error);
}
```

### 3. **双层错误处理架构**

```
Rust Internal Layer (ArithmeticError)
        ↓
   Type Conversion
        ↓
WASM Interface Layer (JsValue)
        ↓
   Error Boundary
        ↓
TypeScript Application Layer (try/catch)
```

**设计目的**：

- **测试友好**：内部层使用 Rust 原生错误类型，支持单元测试
- **互操作性**：接口层转换为 JS 可识别的错误
- **用户体验**：应用层提供友好的错误提示

### 4. **渐进式加载与初始化**

```typescript
// 1. 动态导入（按需加载）
const wasmModule = await import('@rust');

// 2. 异步初始化（避免阻塞主线程）
await wasmModule.default();

// 3. 功能就绪
ArithmeticOperation.calculate(...);
```

**优势**：

- 减少初始加载时间
- 支持代码分割
- 优化用户体验

### 5. **开发与生产环境优化**

```javascript
// webpack.config.js
module.exports = (env, argv) => {
  const isProd = argv.mode === 'production';

  return {
    // 开发模式：快速编译 + 调试友好
    devtool: isProd ? 'source-map' : 'eval-source-map',

    // 生产模式：体积优化 + 性能优化
    optimization: {
      minimize: isProd,
      splitChunks: isProd ? {...} : false,
    }
  };
};
```

### 6. **测试驱动的架构**

```rust
impl ArithmeticOperation {
    // 内部实现：可测试的纯 Rust 函数
    pub fn addition_internal(left: i32, right: i32)
        -> Result<i32, ArithmeticError> {
        left.checked_add(right).ok_or(ArithmeticError::Overflow)
    }

    // WASM 接口：封装内部实现
    #[wasm_bindgen]
    pub fn addition(left: i32, right: i32)
        -> Result<i32, JsValue> {
        Self::addition_internal(left, right)
            .map_err(|_| JsValue::from_str("Overflow"))
    }
}
```

**测试策略**：

- ✅ 单元测试（Rust 原生测试）
- ✅ 集成测试（wasm-bindgen-test）
- ✅ 端到端测试（浏览器环境）

## 🚀 快速开始

```bash
# install dependency
# force yarn
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

### 开发模式

```bash
# 启动开发服务器（支持热更新）
# 如果运行时 rust 没有编译，请移除 node_modules 重新安装
yarn run dev
```

访问：http://localhost:8000

### 生产构建

```bash
# 构建生产版本
yarn run build

# 使用 http-server 预览（需要 Node.js >= 18）
cd dist && http-server -p 8080 .
```

## 📦 技术栈

### 前端技术栈

| 技术           | 版本 | 用途                                |
| -------------- | ---- | ----------------------------------- |
| **Webpack**    | 5.x  | 模块打包工具                        |
| **TypeScript** | 5.x  | 类型安全的 JavaScript               |
| **SWC**        | 1.x  | 快速的 TypeScript/JavaScript 编译器 |
| **SCSS**       | 1.x  | CSS 预处理器                        |

### Rust/WASM 技术栈

| 技术             | 版本         | 用途                   |
| ---------------- | ------------ | ---------------------- |
| **Rust**         | 2021 Edition | 系统编程语言           |
| **wasm-bindgen** | 0.2.x        | Rust/JS 互操作         |
| **wasm-pack**    | 0.12.x       | WASM 构建工具          |
| **wee_alloc**    | 0.4.x        | 轻量级 WASM 内存分配器 |

### 开发工具

| 工具           | 用途           |
| -------------- | -------------- |
| **ESLint**     | 代码质量检查   |
| **Prettier**   | 代码格式化     |
| **Husky**      | Git hooks 管理 |
| **Commitlint** | 提交信息规范   |

## 📁 项目结构

```
webpack5-rust-wasm-template/
├── app/                          # TypeScript 应用代码
│   ├── index.ts                  # 应用入口
│   ├── index.scss                # 样式文件
│   └── index_backup.ts           # 备份文件
├── rust/                         # Rust/WASM 源码
│   ├── src/
│   │   ├── lib.rs                # Rust 库入口
│   │   └── lib_backup.rs         # 备份文件
│   ├── pkg/                      # wasm-pack 编译输出
│   │   ├── index.js              # JS 胶水代码
│   │   ├── index.d.ts            # TypeScript 类型定义
│   │   ├── index_bg.wasm         # WASM 二进制
│   │   └── package.json          # 包信息
│   ├── tests/
│   │   └── wasm.rs               # WASM 集成测试
│   ├── Cargo.toml                # Rust 项目配置
│   └── README.md                 # Rust 项目文档
├── public/                       # 静态资源
│   └── index.html                # HTML 模板
├── dist/                         # 构建输出目录
├── .github/                      # GitHub Actions
│   └── workflows/
│       └── cl.yml                # CI/CD 配置
├── webpack.config.js             # Webpack 配置
├── tsconfig.json                 # TypeScript 配置
├── eslint.config.mjs             # ESLint 配置
├── prettier.config.mjs           # Prettier 配置
├── commitlint.config.js          # Commitlint 配置
├── package.json                  # 项目配置
└── README.md                     # 项目文档
```

## 🔧 详细配置指南

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

WANRING: 如果 MacOS M 系列芯片不能安装成功， 请使用 `cargo install wasm-pack` , 请参考 https://github.com/rustwasm/wasm-pack/issues/952#issuecomment-875585274

```bash
# 不建议全局安装
# 如果wasm-pack 有问题， 请删除 `node_modules`重新安装依赖
yarn add wasm-pack -D

# webpack plugin
yarn add @wasm-tool/wasm-pack-plugin -D

# 创建 rust 项目
cargo new rust --lib

# wasm-pack build --target nodejs
# wasm-pack build --target web
```

在[rust](./rust)中的[Cargo.toml](./rust/Cargo.toml)中添加下面依赖

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

更新[rust/src/lib.rs](./rust/src/lib.rs)

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

## 🔧 详细配置指南

### 1. Webpack 配置

```bash
yarn add webpack webpack-cli webpack-dev-server copy-webpack-plugin -D
```

### 2. TypeScript 配置

```bash
yarn add @swc/core swc-loader typescript -D
```

### 3. SCSS 配置

```bash
yarn add sass css-loader style-loader sass-loader -D
```

查看完整配置：[webpack.config.js](./webpack.config.js)

### 4. wasm-pack 安装

Rust 编译成 WebAssembly 需要 [wasm-pack](https://rustwasm.github.io/wasm-pack/)

⚠️ **注意**：如果 MacOS M 系列芯片不能安装成功，请使用 `cargo install wasm-pack`。
参考：https://github.com/rustwasm/wasm-pack/issues/952#issuecomment-875585274

```bash
# 不建议全局安装
# 如果 wasm-pack 有问题，请删除 node_modules 重新安装依赖
yarn add wasm-pack -D

# 安装 webpack plugin
yarn add @wasm-tool/wasm-pack-plugin -D

# 创建 rust 项目
cargo new rust --lib

# wasm-pack build --target nodejs
# wasm-pack build --target web
```

### 5. Rust 项目配置

在 [rust/Cargo.toml](./rust/Cargo.toml) 中添加以下依赖：

```toml
[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wasm-bindgen = "0.2"

# `wee_alloc` 是 wasm 的微型分配器，代码大小约 1K
# 相比默认分配器的 10K 小很多，但速度稍慢
wee_alloc = { version = "0.4.5", optional = true }

[features]
default = ["wee_alloc"]

[dev-dependencies]
wasm-bindgen-test = "0.3"

[package.metadata.wasm-pack.profile.release]
wasm-opt = ["-Oz", "--enable-mutable-globals"]
```

### 6. Rust 代码示例

更新 [rust/src/lib.rs](./rust/src/lib.rs)：

```rust
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct ArithmeticOperation;

// 内部错误类型（用于测试）
#[derive(Debug, PartialEq)]
pub enum ArithmeticError {
    Overflow,
    DivisionByZero,
}

impl ArithmeticOperation {
    // 内部实现：可测试的纯 Rust 函数
    pub fn addition_internal(left: i32, right: i32) -> Result<i32, ArithmeticError> {
        left.checked_add(right).ok_or(ArithmeticError::Overflow)
    }
}

#[wasm_bindgen]
impl ArithmeticOperation {
    // WASM 接口：暴露给 JavaScript
    #[wasm_bindgen]
    pub fn addition(left: i32, right: i32) -> Result<i32, JsValue> {
        Self::addition_internal(left, right)
            .map_err(|_| JsValue::from_str("Overflow in addition"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let result = ArithmeticOperation::addition_internal(2, 2);
        assert_eq!(result, Ok(4));
    }
}
```

### 7. TypeScript 中使用 WASM

在 [app/index.ts](./app/index.ts) 中引入并初始化 WASM 模块：

```typescript
import type { ArithmeticOperation as ArithmeticOperationType } from '@rust';

interface WasmModule {
  ArithmeticOperation: typeof ArithmeticOperationType;
  default: (module_or_path?: InitInput) => Promise<InitOutput>;
}

class ArithmeticDemo {
  private module: WasmModule | null = null;

  async init(): Promise<void> {
    try {
      // 1. 动态导入 WASM 模块
      this.module = (await import('@rust')) as unknown as WasmModule;

      // 2. 初始化 WASM（必须！）
      await this.module.default();

      // 3. 使用 WASM 函数
      const result = this.module.ArithmeticOperation.addition(1, 5);
      console.log('Result:', result);
    } catch (error) {
      console.error('Failed to load WASM:', error);
    }
  }
}

const demo = new ArithmeticDemo();
demo.init();
```

**重要提示**：

- ⚠️ 使用 `--target web` 模式时，必须先调用 `await wasmModule.default()` 初始化
- ⚠️ 所有 WASM 调用都应该在初始化完成后进行
- ✅ 推荐使用 `try/catch` 包裹 WASM 调用以处理可能的错误

## 🧪 测试

### Rust 单元测试

```bash
cd rust
cargo test
```

### WASM 集成测试

```bash
cd rust
wasm-pack test --headless --firefox
```

### 构建验证

```bash
# 构建并检查输出
yarn run build
ls -la dist/
```

## 📊 性能优化建议

### 1. WASM 代码优化

```toml
# Cargo.toml
[profile.release]
opt-level = "z"     # 优化代码大小
lto = true          # 链接时优化
codegen-units = 1   # 更好的优化，但编译慢
```

### 2. Webpack 优化

```javascript
// webpack.config.js
optimization: {
  splitChunks: {
    cacheGroups: {
      wasm: {
        test: /\.wasm$/,
        name: 'wasm',
        priority: 20,
      },
    },
  },
}
```

### 3. 懒加载策略

```typescript
// 按需加载 WASM 模块
const loadWasm = async () => {
  const module = await import('@rust');
  await module.default();
  return module;
};

// 只在需要时加载
button.onclick = async () => {
  const wasm = await loadWasm();
  const result = wasm.ArithmeticOperation.calculate(...);
};
```

## 🐛 常见问题

### Q1: WASM 模块加载失败

**错误**：`Cannot read properties of undefined`

**解决**：确保在调用 WASM 函数前先初始化模块：

```typescript
const module = await import('@rust');
await module.default(); // 必须调用！
```

### Q2: Rust 编译错误

**错误**：`wasm-pack build failed`

**解决**：

1. 检查 Rust 版本：`rustc --version`
2. 更新工具链：`rustup update`
3. 清理缓存：`cargo clean`

### Q3: 热更新不生效

**解决**：

1. 删除 `node_modules` 和 `rust/target`
2. 重新安装：`yarn install`
3. 重启开发服务器：`yarn run dev`

### Q4: MacOS M 系列芯片安装问题

**解决**：使用 cargo 直接安装 wasm-pack：

```bash
cargo install wasm-pack
```

## 🔗 相关资源

### 官方文档

- [Rust Book](https://doc.rust-lang.org/book/)
- [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/)
- [Webpack Documentation](https://webpack.js.org/)
- [WebAssembly MDN](https://developer.mozilla.org/en-US/docs/WebAssembly)

### 推荐阅读

- [Rust and WebAssembly Book](https://rustwasm.github.io/docs/book/)
- [wasm-pack Documentation](https://rustwasm.github.io/wasm-pack/)
- [WebAssembly Performance](https://hacks.mozilla.org/category/webassembly/)

## 📝 开发规范

### Git Commit 规范

本项目使用 [Conventional Commits](https://www.conventionalcommits.org/)：

```bash
feat: 新功能
fix: 修复 bug
docs: 文档更新
style: 代码格式调整
refactor: 重构
test: 测试相关
chore: 构建/工具链相关
```

### 代码风格

- TypeScript/JavaScript: ESLint + Prettier
- Rust: rustfmt + clippy

```bash
# 格式化代码
yarn run fmt

# 代码检查
yarn run lint

# Rust 格式化
cd rust && cargo fmt

# Rust 检查
cd rust && cargo clippy
```

## 🤝 贡献指南

1. Fork 本仓库
2. 创建特性分支：`git checkout -b feature/amazing-feature`
3. 提交更改：`git commit -m 'feat: add amazing feature'`
4. 推送分支：`git push origin feature/amazing-feature`
5. 提交 Pull Request

## 📄 许可证

MIT License - 查看 [LICENSE](./LICENSE) 文件了解详情

## 🙏 致谢

感谢以下开源项目：

- [Rust](https://www.rust-lang.org/)
- [WebAssembly](https://webassembly.org/)
- [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen)
- [Webpack](https://webpack.js.org/)

## 📮 联系方式

如有问题或建议，欢迎提交 [Issue](https://github.com/freeshineit/webpack5-rust-wasm-template/issues)

---

**Happy Coding! 🎉**
