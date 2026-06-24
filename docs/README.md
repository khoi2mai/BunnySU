#  🐰 BunnySU

<p align="center">
  <img src="https://kernelsu.org/logo.png" width="96" alt="BunnySU Logo">
</p>

<p align="center">
  <strong>A modern kernel-based root solution for Android devices.</strong>
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

## Overview

BunnySU is a kernel-level root solution designed for Android devices.
It provides powerful root access management directly from the kernel layer, offering a clean, efficient, and systemless experience for advanced users, developers, and Android enthusiasts.

Unlike traditional userspace-only root solutions, BunnySU integrates with the kernel to provide deeper control, stronger isolation, and more flexible permission management.

---

## Features

* **Kernel-based root access**
  Provides `su` support directly through the Android kernel.

* **Root permission management**
  Control which apps can access root with a clear and manageable permission system.

* **Systemless module support**
  Supports module-based modifications without directly modifying the system partition.

* **App Profile control**
  Restrict, isolate, or customize root behavior for individual apps.

* **GKI support**
  Designed for modern Android devices using Generic Kernel Image architecture.

* **Advanced compatibility**
  Supports Android phones, WSA, ChromeOS, and container-based Android environments.

---

## Compatibility

BunnySU officially targets Android devices using **GKI 2.0** with kernel **5.10 or newer**.

Older kernels may also work, but they usually require manual kernel integration and device-specific builds.

### Supported architectures

* `arm64-v8a`
* `x86_64`

> [!CAUTION]
> Some recent kernel changes may affect compatibility on certain `x86_64` devices and could cause boot issues or kernel panic.
> Always check device-specific information before flashing or building.

---

## Installation

Please read the official installation guide before using BunnySU.

* [Installation Guide](https://kernelsu.org/guide/installation.html)
* [How to Build](https://kernelsu.org/guide/how-to-build.html)
* [Official Website](https://kernelsu.org/)

> [!WARNING]
> Rooting and modifying the kernel may cause boot failure, data loss, or device instability if done incorrectly.
> Make sure you understand the risks and keep a full backup before proceeding.

---

## Building

BunnySU can be integrated into supported Android kernels and built manually.

For detailed build instructions, please refer to:

* [Build Documentation](https://kernelsu.org/guide/how-to-build.html)

Device maintainers should ensure their kernel source, defconfig, and build environment are properly configured before integration.

---

## Module System

BunnySU supports a systemless module system based on metamodules.

Modules can be used to modify system behavior, add features, or customize the Android environment without directly changing the system partition.

Learn more:

* [Metamodule Documentation](https://kernelsu.org/guide/metamodule.html)

---

## App Profile

App Profile allows fine-grained control over root permissions and app behavior.

With App Profile, users can limit root access, customize privileges, and reduce unnecessary exposure of sensitive root capabilities.

Learn more:

* [App Profile Documentation](https://kernelsu.org/guide/app-profile.html)

---

## Security

BunnySU operates at a low level of the Android system.
Please report security issues responsibly and avoid publicly disclosing vulnerabilities before they are reviewed.

For security-related information, see:

* [Security Policy](SECURITY.md)

---

## License

BunnySU follows the original KernelSU licensing structure:

* Files under the `kernel` directory are licensed under **GPL-2.0-only**.
* Other parts of the project are licensed under **GPL-3.0-or-later**, unless otherwise stated.

See the [LICENSE](LICENSE) file for more details.

---

## Credits

BunnySU is inspired by and built upon the work of the Android root and kernel development community.

Special thanks to:

* [Kernel-Assisted Superuser](https://git.zx2c4.com/kernel-assisted-superuser/about/)
* [Magisk](https://github.com/topjohnwu/Magisk)
* [genuine](https://github.com/brevent/genuine/)
* [Diamorphine](https://github.com/m0nad/Diamorphine)

---

<p align="center">
  <strong>BunnySU</strong><br>
  Kernel-level root access for modern Android devices.
</p>
