# Nexu Protocol
> A distributed causal chain protocol based on the "Cold Truth" design principle, enabling verifiable event tracing through deterministic hash anchoring.

---

## 🎯 项目简介 / Project Introduction
Nexu Protocol 是一个用 Rust 实现的轻量级分布式协议，核心目标是提供**可验证、可复现、无信任依赖**的事件因果链。
它通过 SHA256 哈希锚定每一个事件，确保整个因果链的完整性和确定性，符合「冷真理」设计原则——所有产物哈希可复现，无需依赖第三方信任。

Nexu Protocol is a lightweight distributed protocol implemented in Rust, designed to provide **verifiable, reproducible, trustless** event causal chains.
It anchors each event via SHA256 hash, ensuring the integrity and determinism of the entire causal chain, adhering to the "Cold Truth" principle—all product hashes are reproducible without relying on third-party trust.

## ✨ 核心特性 / Core Features
- **确定性哈希锚定**：每个事件的哈希由前序事件哈希和当前事件内容唯一确定，确保因果链不可篡改。
  **Deterministic Hash Anchoring**: The hash of each event is uniquely determined by the hash of the previous event and the current event content, ensuring the causal chain is immutable.
- **跨语言调用支持**：提供 C-ABI 接口，支持 Go、Python 等多语言 SDK 开发。
  **Cross-Language Support**: Provides a C-ABI interface, supporting SDK development in Go, Python, and other languages.
- **内存安全设计**：内置 `nexu_free` 等内存管理函数，避免跨语言调用时的内存泄漏。
  **Memory-Safe Design**: Built-in memory management functions like `nexu_free` prevent memory leaks during cross-language calls.
- **一键复现构建**：所有产物可通过脚本一键编译，生成的哈希与创世版本完全一致。
  **One-Click Reproducible Build**: All products can be compiled with a single script, generating hashes identical to the genesis version.

## 🚀 快速开始 / Quick Start

### 1. 环境依赖 / Dependencies
- Rust 1.70+
- Go 1.20+
- Git

### 2. 克隆仓库 / Clone Repository
```bash
git clone git@github.com:ffw103888/nexu-protocol.git
cd nexu-protocol
```

### 3. 编译 Rust 核心 / Compile Rust Core
```bash
cd nexu-core
cargo build --release
# 编译产物：target/release/libnexu_core.so
# Build output: target/release/libnexu_core.so
```

### 4. 编译 Go SDK / Compile Go SDK
```bash
cd ../sdk/go
go build -o nexucore.so
```

### 5. 验证创世哈希 / Verify Genesis Hashes
```bash
# 检查 Rust 核心哈希
# Check Rust Core Hash
sha256sum nexu-core/target/release/libnexu_core.so

# 检查 Go SDK 哈希
# Check Go SDK Hash
sha256sum sdk/go/nexucore.so
```
输出的哈希应与 `RELEASE_ASSERTION.txt` 中的创世哈希完全一致。
The output hashes should match exactly the genesis hashes in `RELEASE_ASSERTION.txt`.

## 📚 创世版本锚点 / Genesis Version Anchors
本项目的创世版本 `v2.3-rc1` 已通过以下哈希锚定：
The genesis version `v2.3-rc1` of this project is anchored by the following hashes:
```
Rust Core Hash: bade59e3017c30d31838af857614f8177f00873786181545709e5dd02
C Header Hash: 68793973134457f4c0f4ecfebc94d03eca4f7ee3370a19934b8ddbd927
Go SDK Hash: c0dd7e6872b3a68a04e4b9270c12833a9c013a7baa24cd1b4571252acf
Git Tag Hash: 83cf3b32f052e9ebd32495c0bea7874a542f3e7
```

## 📄 开源协议 / License
本项目采用 [MIT 协议](LICENSE)，允许自由使用、修改和分发。
This project is licensed under the [MIT License](LICENSE), permitting free use, modification, and distribution.

## 🤝 贡献指南 / Contributing
欢迎提交 Issue 和 Pull Request 来改进 Nexu Protocol。
贡献前请阅读 [CONTRIBUTING.md](CONTRIBUTING.md)。

Contributions are welcome via Issues and Pull Requests.
Please read [CONTRIBUTING.md](CONTRIBUTING.md) before contributing.

## 📞 联系方式 / Contact
- 开发者：ffw103888
- 邮箱：1038884359@qq.com
- GitHub：https://github.com/ffw103888/nexu-protocol
