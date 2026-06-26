#  🐰 BunnySU

<p align="center">
  <img src="https://raw.githubusercontent.com/khoi2mai/BunnySU/refs/heads/bunnysu-v3.2.4/docs/assets/bunnysu_logo.png" width="96" alt="BunnySU Logo">
</p>

<p align="center">
  <strong>Современное kernel-based root-решение для Android-устройств.</strong>
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

## Обзор

BunnySU — это root-решение уровня kernel, созданное для Android-устройств.
Оно предоставляет мощное управление root-доступом напрямую из слоя kernel, предлагая чистый, эффективный и systemless-опыт для продвинутых пользователей, разработчиков и энтузиастов Android.

В отличие от традиционных root-решений, работающих только в userspace, BunnySU интегрируется с kernel, чтобы обеспечить более глубокий контроль, более сильную изоляцию и более гибкое управление разрешениями.

---

## Возможности

* **Root-доступ на основе kernel**
  Предоставляет поддержку `su` напрямую через Android kernel.

* **Управление root-разрешениями**
  Контролируйте, какие приложения могут получать root-доступ, с помощью понятной и управляемой системы разрешений.

* **Поддержка systemless-модулей**
  Поддерживает модификации на основе модулей без прямого изменения системного раздела.

* **Управление App Profile**
  Ограничивайте, изолируйте или настраивайте root-поведение для отдельных приложений.

* **Поддержка GKI**
  Разработано для современных Android-устройств, использующих архитектуру Generic Kernel Image.

* **Расширенная совместимость**
  Поддерживает Android-смартфоны, WSA, ChromeOS и Android-среды на основе контейнеров.

---

## Совместимость

BunnySU официально ориентирован на Android-устройства с **GKI 2.0** и kernel **5.10 или новее**.

Старые kernels также могут работать, но обычно требуют ручной интеграции в kernel и сборок, специфичных для устройства.

### Поддерживаемые архитектуры

* `arm64-v8a`
* `x86_64`

> [!CAUTION]
> Некоторые недавние изменения kernel могут повлиять на совместимость отдельных устройств `x86_64` и вызвать проблемы загрузки или kernel panic.
> Всегда проверяйте информацию, относящуюся к вашему устройству, перед прошивкой или сборкой.

---

## Установка

Перед использованием BunnySU прочитайте официальное руководство по установке.

* [Installation Guide](https://kernelsu.org/guide/installation.html)
* [How to Build](https://kernelsu.org/guide/how-to-build.html)
* [Official Website](https://kernelsu.org/)

> [!WARNING]
> Root и изменение kernel при неправильном выполнении могут привести к сбою загрузки, потере данных или нестабильности устройства.
> Убедитесь, что вы понимаете риски, и сделайте полный backup перед продолжением.

---

## Сборка

BunnySU можно интегрировать в поддерживаемые Android kernels и собрать вручную.

Подробные инструкции по сборке смотрите здесь:

* [Build Documentation](https://kernelsu.org/guide/how-to-build.html)

Maintainer'ы устройств должны убедиться, что исходники kernel, defconfig и build environment правильно настроены перед интеграцией.

---

## Система модулей

BunnySU поддерживает systemless module system на основе metamodules.

Modules можно использовать для изменения поведения системы, добавления функций или настройки Android environment без прямого изменения системного раздела.

Подробнее:

* [Metamodule Documentation](https://kernelsu.org/guide/metamodule.html)

---

## App Profile

App Profile позволяет тонко контролировать root-разрешения и поведение приложений.

С помощью App Profile пользователи могут ограничивать root-доступ, настраивать привилегии и уменьшать ненужное раскрытие чувствительных root-возможностей.

Подробнее:

* [App Profile Documentation](https://kernelsu.org/guide/app-profile.html)

---

## Безопасность

BunnySU работает на низком уровне системы Android.
Пожалуйста, сообщайте о проблемах безопасности ответственно и не раскрывайте уязвимости публично до их рассмотрения.

Информацию, связанную с безопасностью, смотрите здесь:

* [Security Policy](SECURITY.md)

---

## Лицензия

BunnySU следует оригинальной структуре лицензирования KernelSU:

* Файлы в директории `kernel` лицензируются по **GPL-2.0-only**.
* Другие части проекта лицензируются по **GPL-3.0-or-later**, если не указано иное.

Подробнее смотрите в файле [LICENSE](LICENSE).

---

## Благодарности

BunnySU вдохновлен работой сообщества Android root и kernel development и построен на ней.

Особая благодарность:

* [Kernel-Assisted Superuser](https://git.zx2c4.com/kernel-assisted-superuser/about/)
* [Magisk](https://github.com/topjohnwu/Magisk)
* [genuine](https://github.com/brevent/genuine/)
* [Diamorphine](https://github.com/m0nad/Diamorphine)

---

<p align="center">
  <strong>BunnySU</strong><br>
  Root-доступ уровня kernel для современных Android-устройств.
</p>
