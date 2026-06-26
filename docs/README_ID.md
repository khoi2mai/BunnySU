# 🐰 BunnySU

<p align="center">
  <img src="https://raw.githubusercontent.com/khoi2mai/BunnySU/refs/heads/bunnysu/docs/assets/bunnysu_logo.png" width="96" alt="BunnySU Logo">
</p>

<p align="center">
  <strong>Solusi root modern berbasis kernel untuk perangkat Android.</strong>
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

## Gambaran Umum

BunnySU adalah solusi root tingkat kernel yang dirancang untuk perangkat Android.
BunnySU menyediakan manajemen akses root yang kuat langsung dari lapisan kernel, sehingga memberikan pengalaman yang bersih, efisien, dan systemless untuk pengguna tingkat lanjut, developer, serta penggemar Android.

Berbeda dari solusi root tradisional yang hanya berjalan di userspace, BunnySU terintegrasi dengan kernel untuk memberikan kontrol yang lebih dalam, isolasi yang lebih kuat, dan manajemen izin yang lebih fleksibel.

---

## Fitur

* **Akses root berbasis kernel**
  Menyediakan dukungan `su` langsung melalui kernel Android.

* **Manajemen izin root**
  Mengontrol aplikasi mana saja yang dapat mengakses root melalui sistem izin yang jelas dan mudah dikelola.

* **Dukungan modul systemless**
  Mendukung modifikasi berbasis modul tanpa mengubah partisi sistem secara langsung.

* **Kontrol App Profile**
  Membatasi, mengisolasi, atau menyesuaikan perilaku root untuk setiap aplikasi secara terpisah.

* **Dukungan GKI**
  Dirancang untuk perangkat Android modern yang menggunakan arsitektur Generic Kernel Image.

* **Kompatibilitas lanjutan**
  Mendukung ponsel Android, WSA, ChromeOS, dan lingkungan Android berbasis container.

---

## Kompatibilitas

BunnySU secara resmi menargetkan perangkat Android yang menggunakan **GKI 2.0** dengan kernel **5.10 atau lebih baru**.

Kernel yang lebih lama mungkin juga dapat digunakan, tetapi biasanya memerlukan integrasi kernel manual dan build khusus untuk perangkat tertentu.

### Arsitektur yang didukung

* `arm64-v8a`
* `x86_64`

> [!CAUTION]
> Beberapa perubahan kernel terbaru dapat memengaruhi kompatibilitas pada perangkat `x86_64` tertentu dan dapat menyebabkan masalah boot atau kernel panic.
> Selalu periksa informasi khusus perangkat sebelum melakukan flashing atau build.

---

## Instalasi

Silakan baca panduan instalasi resmi sebelum menggunakan BunnySU.

* [Panduan Instalasi](https://kernelsu.org/guide/installation.html)
* [Cara Build](https://kernelsu.org/guide/how-to-build.html)
* [Situs Web Resmi](https://kernelsu.org/)

> [!WARNING]
> Rooting dan modifikasi kernel dapat menyebabkan gagal boot, kehilangan data, atau ketidakstabilan perangkat jika dilakukan dengan tidak benar.
> Pastikan Anda memahami risikonya dan membuat cadangan penuh sebelum melanjutkan.

---

## Build

BunnySU dapat diintegrasikan ke dalam kernel Android yang didukung dan dibangun secara manual.

Untuk instruksi build yang lebih detail, silakan lihat:

* [Dokumentasi Build](https://kernelsu.org/guide/how-to-build.html)

Maintainer perangkat harus memastikan source kernel, defconfig, dan lingkungan build sudah dikonfigurasi dengan benar sebelum integrasi.

---

## Sistem Modul

BunnySU mendukung sistem modul systemless berbasis metamodule.

Modul dapat digunakan untuk mengubah perilaku sistem, menambahkan fitur, atau menyesuaikan lingkungan Android tanpa mengubah partisi sistem secara langsung.

Pelajari lebih lanjut:

* [Dokumentasi Metamodule](https://kernelsu.org/guide/metamodule.html)

---

## App Profile

App Profile memungkinkan kontrol yang lebih detail terhadap izin root dan perilaku aplikasi.

Dengan App Profile, pengguna dapat membatasi akses root, menyesuaikan hak istimewa, dan mengurangi paparan kemampuan root yang sensitif yang tidak diperlukan.

Pelajari lebih lanjut:

* [Dokumentasi App Profile](https://kernelsu.org/guide/app-profile.html)

---

## Keamanan

BunnySU bekerja pada level rendah dari sistem Android.
Harap laporkan masalah keamanan secara bertanggung jawab dan hindari mengungkapkan kerentanan secara publik sebelum ditinjau.

Untuk informasi terkait keamanan, lihat:

* [Kebijakan Keamanan](SECURITY.md)

---

## Lisensi

BunnySU mengikuti struktur lisensi asli KernelSU:

* File di dalam direktori `kernel` dilisensikan di bawah **GPL-2.0-only**.
* Bagian lain dari proyek ini dilisensikan di bawah **GPL-3.0-or-later**, kecuali dinyatakan lain.

Lihat file [LICENSE](LICENSE) untuk detail lebih lanjut.

---

## Kredit

BunnySU terinspirasi dari dan dibangun di atas karya komunitas pengembangan root dan kernel Android.

Terima kasih khusus kepada:

* [Kernel-Assisted Superuser](https://git.zx2c4.com/kernel-assisted-superuser/about/)
* [Magisk](https://github.com/topjohnwu/Magisk)
* [genuine](https://github.com/brevent/genuine/)
* [Diamorphine](https://github.com/m0nad/Diamorphine)

---

<p align="center">
  <strong>BunnySU</strong><br>
  Akses root tingkat kernel untuk perangkat Android modern.
</p>
