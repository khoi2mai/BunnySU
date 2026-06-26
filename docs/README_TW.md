#  🐰 BunnySU

<p align="center">
  <img src="https://raw.githubusercontent.com/khoi2mai/BunnySU/refs/heads/bunnysu-v3.2.4/docs/assets/bunnysu_logo.png" width="96" alt="BunnySU Logo">
</p>

<p align="center">
  <strong>適用於 Android 裝置的現代 kernel-based root 解決方案。</strong>
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
  <a href="README_ID.md">Indonesia</a> |
  <a href="README_IN.md">हिन्दी</a> |
  <a href="README_IT.md">Italiano</a> |
  <a href="README_IW.md">עברית</a>
</p>

---

## 概述

BunnySU 是專為 Android 裝置設計的 kernel-level root 解決方案。
它直接從 kernel layer 提供強大的 root access management，為進階使用者、開發者與 Android 愛好者帶來乾淨、高效率且 systemless 的體驗。

不同於傳統僅依賴 userspace 的 root solutions，BunnySU 與 kernel 整合，提供更深入的控制、更強的隔離能力，以及更彈性的 permission management。

---

## 功能

* **Kernel-based root access**
  直接透過 Android kernel 提供 `su` support。

* **Root permission management**
  透過清楚且易於管理的 permission system，控制哪些 apps 可以取得 root access。

* **Systemless module support**
  支援 module-based modifications，不需要直接修改 system partition。

* **App Profile control**
  可針對個別 apps 限制、隔離或自訂 root behavior。

* **GKI support**
  專為使用 Generic Kernel Image architecture 的 modern Android devices 而設計。

* **Advanced compatibility**
  支援 Android phones、WSA、ChromeOS，以及 container-based Android environments。

---

## 相容性

BunnySU 官方目標為使用 **GKI 2.0** 且 kernel **5.10 或更新版本** 的 Android 裝置。

較舊的 kernels 也可能可以運作，但通常需要 manual kernel integration 與 device-specific builds。

### 支援架構

* `arm64-v8a`
* `x86_64`

> [!CAUTION]
> 部分近期的 kernel changes 可能會影響某些 `x86_64` devices 的相容性，並可能導致 boot issues 或 kernel panic。
> 在 flash 或 build 前，請務必確認 device-specific information。

---

## 安裝

使用 BunnySU 前，請先閱讀官方 installation guide。

* [Installation Guide](https://kernelsu.org/guide/installation.html)
* [How to Build](https://kernelsu.org/guide/how-to-build.html)
* [Official Website](https://kernelsu.org/)

> [!WARNING]
> Rooting 與修改 kernel 若操作不當，可能造成 boot failure、data loss 或 device instability。
> 繼續操作前，請確認你了解風險並保留完整備份。

---

## 建置

BunnySU 可以整合到受支援的 Android kernels 中並手動 build。

詳細 build instructions 請參考：

* [Build Documentation](https://kernelsu.org/guide/how-to-build.html)

Device maintainers 在整合前應確認 kernel source、defconfig 與 build environment 已正確設定。

---

## Module System

BunnySU 支援基於 metamodules 的 systemless module system。

Modules 可用於修改 system behavior、新增 features，或在不直接修改 system partition 的情況下自訂 Android environment。

了解更多：

* [Metamodule Documentation](https://kernelsu.org/guide/metamodule.html)

---

## App Profile

App Profile 可對 root permissions 與 app behavior 進行細緻控制。

透過 App Profile，users 可以限制 root access、自訂 privileges，並減少敏感 root capabilities 的不必要暴露。

了解更多：

* [App Profile Documentation](https://kernelsu.org/guide/app-profile.html)

---

## Security

BunnySU 運作於 Android system 的低層級。
請負責任地回報 security issues，並避免在漏洞完成 review 前公開 disclosed。

安全相關資訊請參考：

* [Security Policy](SECURITY.md)

---

## License

BunnySU 遵循 original KernelSU licensing structure：

* `kernel` directory 下的 files 採用 **GPL-2.0-only** 授權。
* Project 的其他部分採用 **GPL-3.0-or-later** 授權，除非另有說明。

更多詳細資訊請參閱 [LICENSE](LICENSE) file。

---

## Credits

BunnySU 受到 Android root 與 kernel development community 工作的啟發並以其成果為基礎。

Special thanks to:

* [Kernel-Assisted Superuser](https://git.zx2c4.com/kernel-assisted-superuser/about/)
* [Magisk](https://github.com/topjohnwu/Magisk)
* [genuine](https://github.com/brevent/genuine/)
* [Diamorphine](https://github.com/m0nad/Diamorphine)

---

<p align="center">
  <strong>BunnySU</strong><br>
  適用於 modern Android devices 的 kernel-level root access。
</p>
