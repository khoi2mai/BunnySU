#  🐰 BunnySU

<p align="center">
  <img src="https://raw.githubusercontent.com/khoi2mai/BunnySU/refs/heads/bunnysu/docs/assets/bunnysu_logo.png" width="96" alt="BunnySU Logo">
</p>

<p align="center">
  <strong>面向 Android 设备的现代化内核级 Root 解决方案。</strong>
</p>

<p align="center">
  <a href="https://github.com/khoi2mai/BunnySU/releases/latest">
    <img src="https://img.shields.io/github/v/release/khoi2mai/BunnySU?label=Release&logo=github" alt="Latest Release">
  </a>
  <a href="/LICENSE">
    <img src="https://img.shields.io/github/license/khoi2mai/BunnySU?logo=gnu" alt="License">
  </a>
  <a href="https://t.me/KernelSU">
    <img src="https://img.shields.io/badge/Telegram-Community-blue?logo=telegram" alt="Telegram">
  </a>
</p>

<p align="center">
  <a href="README.md">English</a> |
  <a href="README_ES.md">Español</a> |
  <a href="README_CN.md">简体中文</a> |
  <a href="README_TW.md">繁體中文</a> |
  <a href="README_JP.md">日本語</a> |
  <a href="README_KR.md">한국어</a> |
  <a href="README_PL.md">Polski</a> |
  <a href="README_PT-BR.md">Português</a> |
  <a href="README_TR.md">Türkçe</a> |
  <a href="README_RU.md">Русский</a> |
  <a href="README_VI.md">Tiếng Việt</a> |
  <a href="README_ID.md">Indonesia</a>
</p>

---

## 概述

BunnySU 是一款为 Android 设备设计的内核级 Root 解决方案。
它直接从内核层提供强大的 Root 权限管理能力，为高级用户、开发者以及 Android 爱好者带来干净、高效且 systemless 的使用体验。

不同于传统仅依赖用户空间的 Root 方案，BunnySU 与内核集成，可提供更深层的控制能力、更强的隔离性，以及更加灵活的权限管理。

---

## 功能特性

* **基于内核的 Root 访问**
  通过 Android 内核直接提供 `su` 支持。

* **Root 权限管理**
  通过清晰且易于管理的权限系统，控制哪些应用可以访问 Root 权限。

* **Systemless 模块支持**
  支持基于模块的修改方式，无需直接修改 system 分区。

* **App Profile 控制**
  可针对单个应用限制、隔离或自定义 Root 行为。

* **GKI 支持**
  面向使用 Generic Kernel Image 架构的现代 Android 设备设计。

* **高级兼容性**
  支持 Android 手机、WSA、ChromeOS 以及基于容器的 Android 环境。

---

## 兼容性

BunnySU 官方目标是使用 **GKI 2.0** 且内核版本为 **5.10 或更新版本** 的 Android 设备。

较旧版本的内核也可能可以工作，但通常需要手动进行内核集成，并为具体设备构建对应版本。

### 支持的架构

* `arm64-v8a`
* `x86_64`

> [!CAUTION]
> 某些近期的内核改动可能会影响部分 `x86_64` 设备的兼容性，并可能导致无法启动或 kernel panic。
> 在刷入或构建之前，请务必先确认你的设备相关信息。

---

## 安装

在使用 BunnySU 之前，请先阅读官方安装指南。

* [安装指南](https://kernelsu.org/guide/installation.html)
* [如何构建](https://kernelsu.org/guide/how-to-build.html)
* [官方网站](https://kernelsu.org/)

> [!WARNING]
> Root 和修改内核操作如果执行不当，可能会导致无法启动、数据丢失或设备不稳定。
> 请确保你已了解相关风险，并在继续之前做好完整备份。

---

## 构建

BunnySU 可以集成到受支持的 Android 内核中，并进行手动构建。

详细构建说明请参考：

* [构建文档](https://kernelsu.org/guide/how-to-build.html)

设备维护者应确保其内核源码、defconfig 以及构建环境已正确配置，然后再进行集成。

---

## 模块系统

BunnySU 支持基于 metamodules 的 systemless 模块系统。

模块可用于修改系统行为、添加功能，或自定义 Android 环境，而无需直接更改 system 分区。

了解更多：

* [Metamodule 文档](https://kernelsu.org/guide/metamodule.html)

---

## App Profile

App Profile 允许对 Root 权限和应用行为进行精细化控制。

通过 App Profile，用户可以限制 Root 访问、自定义权限，并减少敏感 Root 能力的不必要暴露。

了解更多：

* [App Profile 文档](https://kernelsu.org/guide/app-profile.html)

---

## 安全

BunnySU 运行在 Android 系统的底层。
请负责任地报告安全问题，并避免在问题被审查之前公开披露漏洞。

安全相关信息请参阅：

* [安全策略](SECURITY.md)

---

## 许可证

BunnySU 遵循原 KernelSU 的许可证结构：

* `kernel` 目录下的文件使用 **GPL-2.0-only** 许可证。
* 除非另有说明，项目的其他部分使用 **GPL-3.0-or-later** 许可证。

更多详细信息请参阅 [LICENSE](LICENSE) 文件。

---

## 致谢

BunnySU 受到 Android Root 与内核开发社区工作的启发，并基于其成果构建。

特别感谢：

* [Kernel-Assisted Superuser](https://git.zx2c4.com/kernel-assisted-superuser/about/)
* [Magisk](https://github.com/topjohnwu/Magisk)
* [genuine](https://github.com/brevent/genuine/)
* [Diamorphine](https://github.com/m0nad/Diamorphine)

---

<p align="center">
  <strong>BunnySU</strong><br>
  面向现代 Android 设备的内核级 Root 访问方案。
</p>
