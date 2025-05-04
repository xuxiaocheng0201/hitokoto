# Hitokoto

[![Crate](https://img.shields.io/crates/v/hitokoto.svg)](https://crates.io/crates/hitokoto)
[![GitHub last commit](https://img.shields.io/github/last-commit/xuxiaocheng0201/hitokoto)](https://github.com/xuxiaocheng0201/hitokoto/commits/master)
[![GitHub issues](https://img.shields.io/github/issues-raw/xuxiaocheng0201/hitokoto)](https://github.com/xuxiaocheng0201/hitokoto/issues)
[![GitHub pull requests](https://img.shields.io/github/issues-pr/xuxiaocheng0201/hitokoto)](https://github.com/xuxiaocheng0201/hitokoto/pulls)

# Description

关于[一言](https://hitokoto.cn/about)：
> 动漫也好、小说也好、网络也好，不论在哪里，我们总会看到有那么一两个句子能穿透你的心。我们把这些句子汇聚起来，形成一言网络，以传递更多的感动。如果可以，我们希望我们没有停止服务的那一天。
> 
> 简单来说，一言指的就是一句话，可以是动漫中的台词，也可以是网络上的各种小段子。 或是感动，或是开心，有或是单纯的回忆。来到这里，留下你所喜欢的那一句句话，与大家分享，这就是一言存在的目的。

# Features

1. 构建时自动下载并捆绑所有一言，最新版本可在 Github 中查看 [Tags](https://github.com/hitokoto-osc/sentences-bundle/tags)
2. 支持随机选择，与访问 https://v1.hitokoto.cn/?c= 相同 (需要 `random` feature，默认开启)
3. 在 API 的基础上扩展支持了语言，使用 [lingua](https://crates.io/crates/lingua) 库在构建时自动识别 (需要 `language` feature)
4. 支持 no_std
5. 支持 `wasm32-wasip1`
