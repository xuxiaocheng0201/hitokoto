# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.1] - 2025-1-13

### Added

* commit_from 字段使用枚举
* 添加 get_hitokoto_by_id 函数

## [0.4.0] - 2025-1-13

### Added

* 内置识别语言

## [0.3.0] - 2025-1-12

### Changed

* created_at 字段类型改为 `chrono::DateTime<chrono::Utc>`

## [0.2.2] - 2025-1-12

### Added

* 支持 no_std 环境

### Fixed

* 优化 docs.rs 文档

## [0.1.0] - 2025-1-9

### Added

* 捆绑语句库，支持随机选择
