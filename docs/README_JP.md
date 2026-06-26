#  🐰 BunnySU

<p align="center">
  <img src="https://raw.githubusercontent.com/khoi2mai/BunnySU/refs/heads/bunnysu/docs/assets/bunnysu_logo.png" width="96" alt="BunnySU Logo">
</p>

<p align="center">
  <strong>Android デバイス向けの最新 kernel-based root ソリューション。</strong>
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

## 概要

BunnySU は、Android デバイス向けに設計された kernel-level root ソリューションです。
kernel layer から直接強力な root access management を提供し、advanced users、developers、Android enthusiasts に向けて、clean で efficient な systemless 体験を実現します。

従来の userspace-only root solutions とは異なり、BunnySU は kernel と統合することで、より深い制御、より強力な分離、より柔軟な permission management を提供します。

---

## 機能

* **Kernel-based root access**
  Android kernel を通じて直接 `su` support を提供します。

* **Root permission management**
  明確で管理しやすい permission system により、どの apps が root にアクセスできるかを制御できます。

* **Systemless module support**
  system partition を直接変更せずに、module-based modifications をサポートします。

* **App Profile control**
  個別の apps ごとに root behavior を制限、分離、またはカスタマイズできます。

* **GKI support**
  Generic Kernel Image architecture を使用する modern Android devices 向けに設計されています。

* **Advanced compatibility**
  Android phones、WSA、ChromeOS、container-based Android environments をサポートします。

---

## 互換性

BunnySU は公式には **GKI 2.0** と kernel **5.10 以降** を使用する Android デバイスを対象としています。

古い kernels でも動作する可能性がありますが、通常は manual kernel integration と device-specific builds が必要です。

### サポートされるアーキテクチャ

* `arm64-v8a`
* `x86_64`

> [!CAUTION]
> 最近の kernel changes により、一部の `x86_64` devices で互換性に影響が出る場合があり、boot issues や kernel panic が発生する可能性があります。
> Flash または build する前に、必ず device-specific information を確認してください。

---

## インストール

BunnySU を使用する前に、公式の installation guide を読んでください。

* [Installation Guide](https://kernelsu.org/guide/installation.html)
* [How to Build](https://kernelsu.org/guide/how-to-build.html)
* [Official Website](https://kernelsu.org/)

> [!WARNING]
> Rooting や kernel の変更を誤って行うと、boot failure、data loss、または device instability が発生する可能性があります。
> 作業を進める前にリスクを理解し、full backup を保持してください。

---

## ビルド

BunnySU は supported Android kernels に統合し、手動で build できます。

詳細な build instructions については、以下を参照してください。

* [Build Documentation](https://kernelsu.org/guide/how-to-build.html)

Device maintainers は、integration の前に kernel source、defconfig、build environment が正しく構成されていることを確認してください。

---

## Module System

BunnySU は metamodules に基づく systemless module system をサポートします。

Modules は、system partition を直接変更せずに、system behavior の変更、features の追加、Android environment のカスタマイズに使用できます。

詳細:

* [Metamodule Documentation](https://kernelsu.org/guide/metamodule.html)

---

## App Profile

App Profile は root permissions と app behavior を細かく制御できます。

App Profile により、users は root access を制限し、privileges をカスタマイズし、sensitive root capabilities の不要な露出を減らすことができます。

詳細:

* [App Profile Documentation](https://kernelsu.org/guide/app-profile.html)

---

## Security

BunnySU は Android system の低レベルで動作します。
Security issues は責任を持って報告し、review される前に vulnerabilities を公開しないでください。

Security-related information については、以下を参照してください。

* [Security Policy](SECURITY.md)

---

## License

BunnySU は original KernelSU licensing structure に従います。

* `kernel` directory 以下の files は **GPL-2.0-only** の下でライセンスされています。
* Project のその他の部分は、別途明記されていない限り **GPL-3.0-or-later** の下でライセンスされています。

詳細は [LICENSE](LICENSE) file を参照してください。

---

## Credits

BunnySU は Android root と kernel development community の成果に着想を得て構築されています。

Special thanks to:

* [Kernel-Assisted Superuser](https://git.zx2c4.com/kernel-assisted-superuser/about/)
* [Magisk](https://github.com/topjohnwu/Magisk)
* [genuine](https://github.com/brevent/genuine/)
* [Diamorphine](https://github.com/m0nad/Diamorphine)

---

<p align="center">
  <strong>BunnySU</strong><br>
  Modern Android devices 向けの kernel-level root access.
</p>
