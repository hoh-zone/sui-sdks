# Sui SDKs（非官方）

[English](./README.md) | 简体中文

这是一个面向 **Sui 生态** 的多语言 SDK 仓库，目标是提供不同编程语言下的 Sui 开发能力实现与工程实践。

## 项目定位

- 非官方实现：本仓库为社区/个人维护，**不隶属于 Mysten Labs 或 Sui Foundation 官方 SDK 发布渠道**。
- 多语言覆盖：统一维护多种语言 SDK，便于跨语言对照、迁移与能力补齐。
- 工程化优先：聚焦可用性、测试与文档，支持在不同技术栈中快速接入 Sui。

## 仓库结构

- `ts-sdks/`：TypeScript SDK（当前保留独立 Git 历史）
- `go-sdks/`：Go SDK
- `py-sdks/`：Python SDK
- `java-sdks/`：Java SDK
- `kotlin-sdks/`：Kotlin SDK
- `rust-sdks/`：Rust SDK
- `swift-sdks/`：Swift SDK
- `dart-sdks/`：Dart SDK

## 文档站点

- 顶层文档站：`docs/`
- 本地运行：`cd docs && npm install && npm run dev`

## 快速开始

进入对应语言目录，查看各自的 `README.md` 获取安装、构建与测试说明。

## 免责声明

本项目为非官方实现，仅供学习、研究与工程实践参考。
在生产环境使用前，请自行完成安全审计、兼容性验证和风险评估。

## 贡献

欢迎提交 Issue / PR，共同完善各语言 SDK 的功能一致性、文档质量与测试覆盖。
