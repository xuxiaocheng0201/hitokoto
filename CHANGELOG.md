# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.7.1] - 2025-5-16

### Fixed

* 修复因为 `zip` yanked 导致的错误

## [0.7.0] - 2025-5-5

### Added

* 将原来的 `language` feature 重命名为 `language-full`
* 添加简化版的新的 `language` feature，仅识别有限种语言

## [0.6.0] - 2025-5-5

### Added

* 添加 `bundled_version` 函数
* 添加 `get_all_hitokoto` 函数

### Changed

* 更新 `lingua` 到 1.7
* 重构模块导出结构
* 从 `api.github.com` 下载语句包

## [0.5.2] - 2025-2-7

### Changed

* 更新 `rand` 到 0.9

## [0.5.1] - 2025-1-13

### Added

* `commit_from` 字段使用枚举
* 添加 `get_hitokoto_by_id` 函数

## [0.4.0] - 2025-1-13

### Added

* 内置识别语言

## [0.3.0] - 2025-1-12

### Changed

* `created_at` 字段类型改为 `chrono::DateTime<chrono::Utc>`

## [0.2.2] - 2025-1-12

### Added

* 支持 no_std 环境

### Fixed

* 优化 docs.rs 文档

## [0.1.0] - 2025-1-9

### Added

* 捆绑语句库，支持随机选择
