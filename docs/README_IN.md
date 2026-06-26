#  🐰 BunnySU

<p align="center">
  <img src="https://raw.githubusercontent.com/khoi2mai/BunnySU/refs/heads/bunnysu/docs/assets/bunnysu_logo.png" width="96" alt="BunnySU Logo">
</p>

<p align="center">
  <strong>Android उपकरणों के लिए एक आधुनिक kernel-based root समाधान।</strong>
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

## अवलोकन

BunnySU Android उपकरणों के लिए बनाया गया एक kernel-level root समाधान है।
यह kernel layer से सीधे शक्तिशाली root access management प्रदान करता है, जिससे advanced users, developers और Android enthusiasts के लिए एक clean, efficient और systemless अनुभव मिलता है।

Traditional userspace-only root solutions से अलग, BunnySU kernel के साथ integrate होकर deeper control, stronger isolation और अधिक flexible permission management प्रदान करता है।

---

## विशेषताएँ

* **Kernel-based root access**
  Android kernel के माध्यम से सीधे `su` support प्रदान करता है।

* **Root permission management**
  एक clear और manageable permission system के साथ control करें कि कौन-से apps root access कर सकते हैं।

* **Systemless module support**
  system partition को सीधे modify किए बिना module-based modifications support करता है।

* **App Profile control**
  अलग-अलग apps के लिए root behavior को restrict, isolate या customize करें।

* **GKI support**
  Generic Kernel Image architecture इस्तेमाल करने वाले modern Android devices के लिए design किया गया है।

* **Advanced compatibility**
  Android phones, WSA, ChromeOS और container-based Android environments को support करता है।

---

## Compatibility

BunnySU officially उन Android devices को target करता है जो **GKI 2.0** और kernel **5.10 या newer** का उपयोग करते हैं।

Older kernels भी काम कर सकते हैं, लेकिन आमतौर पर उन्हें manual kernel integration और device-specific builds की जरूरत होती है।

### Supported architectures

* `arm64-v8a`
* `x86_64`

> [!CAUTION]
> कुछ recent kernel changes कुछ `x86_64` devices पर compatibility को affect कर सकते हैं और boot issues या kernel panic का कारण बन सकते हैं।
> Flash या build करने से पहले हमेशा device-specific information check करें।

---

## Installation

BunnySU इस्तेमाल करने से पहले official installation guide पढ़ें।

* [Installation Guide](https://kernelsu.org/guide/installation.html)
* [How to Build](https://kernelsu.org/guide/how-to-build.html)
* [Official Website](https://kernelsu.org/)

> [!WARNING]
> Rooting और kernel modification गलत तरीके से करने पर boot failure, data loss या device instability हो सकती है।
> आगे बढ़ने से पहले risks समझें और full backup रखें।

---

## Building

BunnySU को supported Android kernels में integrate करके manually build किया जा सकता है।

Detailed build instructions के लिए देखें:

* [Build Documentation](https://kernelsu.org/guide/how-to-build.html)

Device maintainers को integration से पहले यह सुनिश्चित करना चाहिए कि kernel source, defconfig और build environment properly configured हों।

---

## Module System

BunnySU metamodules पर आधारित systemless module system support करता है।

Modules का उपयोग system behavior modify करने, features add करने या system partition को सीधे बदले बिना Android environment customize करने के लिए किया जा सकता है।

और जानें:

* [Metamodule Documentation](https://kernelsu.org/guide/metamodule.html)

---

## App Profile

App Profile root permissions और app behavior पर fine-grained control देता है।

App Profile के साथ users root access limit कर सकते हैं, privileges customize कर सकते हैं और sensitive root capabilities की unnecessary exposure कम कर सकते हैं।

और जानें:

* [App Profile Documentation](https://kernelsu.org/guide/app-profile.html)

---

## Security

BunnySU Android system के low level पर operate करता है।
कृपया security issues को responsibly report करें और vulnerabilities review होने से पहले public disclose न करें।

Security-related information के लिए देखें:

* [Security Policy](SECURITY.md)

---

## License

BunnySU original KernelSU licensing structure को follow करता है:

* `kernel` directory के अंतर्गत files **GPL-2.0-only** license के तहत हैं।
* Project के अन्य हिस्से **GPL-3.0-or-later** license के तहत हैं, जब तक अलग से stated न हो।

अधिक जानकारी के लिए [LICENSE](LICENSE) file देखें।

---

## Credits

BunnySU Android root और kernel development community के काम से inspired है और उसी पर built है।

Special thanks to:

* [Kernel-Assisted Superuser](https://git.zx2c4.com/kernel-assisted-superuser/about/)
* [Magisk](https://github.com/topjohnwu/Magisk)
* [genuine](https://github.com/brevent/genuine/)
* [Diamorphine](https://github.com/m0nad/Diamorphine)

---

<p align="center">
  <strong>BunnySU</strong><br>
  Modern Android devices के लिए kernel-level root access.
</p>
