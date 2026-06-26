#  🐰 BunnySU

<p align="center">
  <img src="https://raw.githubusercontent.com/khoi2mai/BunnySU/refs/heads/bunnysu/docs/assets/bunnysu_logo.png" width="96" alt="BunnySU Logo">
</p>

<p align="center">
  <strong>Android 기기를 위한 최신 kernel-based root 솔루션.</strong>
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

## 개요

BunnySU는 Android 기기를 위해 설계된 kernel-level root 솔루션입니다.
kernel layer에서 직접 강력한 root access management를 제공하여 advanced users, developers, Android enthusiasts에게 clean, efficient, systemless 경험을 제공합니다.

기존의 userspace-only root solutions와 달리 BunnySU는 kernel과 통합되어 더 깊은 제어, 더 강력한 격리, 더 유연한 permission management를 제공합니다.

---

## 기능

* **Kernel-based root access**
  Android kernel을 통해 직접 `su` support를 제공합니다.

* **Root permission management**
  명확하고 관리하기 쉬운 permission system으로 어떤 apps가 root에 접근할 수 있는지 제어합니다.

* **Systemless module support**
  system partition을 직접 수정하지 않고 module-based modifications를 지원합니다.

* **App Profile control**
  개별 apps에 대해 root behavior를 제한, 격리 또는 사용자 지정할 수 있습니다.

* **GKI support**
  Generic Kernel Image architecture를 사용하는 modern Android devices를 위해 설계되었습니다.

* **Advanced compatibility**
  Android phones, WSA, ChromeOS, container-based Android environments를 지원합니다.

---

## 호환성

BunnySU는 공식적으로 **GKI 2.0** 및 kernel **5.10 이상**을 사용하는 Android 기기를 대상으로 합니다.

Older kernels도 동작할 수 있지만, 일반적으로 manual kernel integration과 device-specific builds가 필요합니다.

### 지원 아키텍처

* `arm64-v8a`
* `x86_64`

> [!CAUTION]
> 일부 recent kernel changes는 특정 `x86_64` devices의 호환성에 영향을 줄 수 있으며 boot issues 또는 kernel panic을 유발할 수 있습니다.
> Flash 또는 build하기 전에 항상 device-specific information을 확인하세요.

---

## 설치

BunnySU를 사용하기 전에 official installation guide를 읽어 주세요.

* [Installation Guide](https://kernelsu.org/guide/installation.html)
* [How to Build](https://kernelsu.org/guide/how-to-build.html)
* [Official Website](https://kernelsu.org/)

> [!WARNING]
> Rooting 및 kernel modification을 잘못 수행하면 boot failure, data loss 또는 device instability가 발생할 수 있습니다.
> 진행하기 전에 위험을 이해하고 full backup을 보관하세요.

---

## 빌드

BunnySU는 supported Android kernels에 통합하여 manually build할 수 있습니다.

자세한 build instructions는 다음을 참조하세요.

* [Build Documentation](https://kernelsu.org/guide/how-to-build.html)

Device maintainers는 integration 전에 kernel source, defconfig, build environment가 올바르게 구성되어 있는지 확인해야 합니다.

---

## Module System

BunnySU는 metamodules 기반의 systemless module system을 지원합니다.

Modules는 system partition을 직접 변경하지 않고 system behavior를 수정하거나 features를 추가하거나 Android environment를 사용자 지정하는 데 사용할 수 있습니다.

자세히 알아보기:

* [Metamodule Documentation](https://kernelsu.org/guide/metamodule.html)

---

## App Profile

App Profile은 root permissions와 app behavior를 세밀하게 제어할 수 있게 해줍니다.

App Profile을 통해 users는 root access를 제한하고 privileges를 사용자 지정하며 sensitive root capabilities의 불필요한 노출을 줄일 수 있습니다.

자세히 알아보기:

* [App Profile Documentation](https://kernelsu.org/guide/app-profile.html)

---

## Security

BunnySU는 Android system의 낮은 수준에서 동작합니다.
Security issues는 책임감 있게 report하고, vulnerabilities가 검토되기 전에 공개적으로 disclosed하지 마세요.

Security-related information은 다음을 참조하세요.

* [Security Policy](SECURITY.md)

---

## License

BunnySU는 original KernelSU licensing structure를 따릅니다.

* `kernel` directory 아래의 files는 **GPL-2.0-only**로 라이선스됩니다.
* Project의 다른 부분은 별도로 명시되지 않는 한 **GPL-3.0-or-later**로 라이선스됩니다.

자세한 내용은 [LICENSE](LICENSE) file을 참조하세요.

---

## Credits

BunnySU는 Android root 및 kernel development community의 작업에서 영감을 받고 이를 기반으로 만들어졌습니다.

Special thanks to:

* [Kernel-Assisted Superuser](https://git.zx2c4.com/kernel-assisted-superuser/about/)
* [Magisk](https://github.com/topjohnwu/Magisk)
* [genuine](https://github.com/brevent/genuine/)
* [Diamorphine](https://github.com/m0nad/Diamorphine)

---

<p align="center">
  <strong>BunnySU</strong><br>
  Modern Android devices를 위한 kernel-level root access.
</p>
