#  🐰 BunnySU

<p align="center">
  <img src="https://raw.githubusercontent.com/khoi2mai/BunnySU/refs/heads/bunnysu-v3.2.4/docs/assets/bunnysu_logo.png" width="96" alt="BunnySU Logo">
</p>

<p align="center">
  <strong>Giải pháp root hiện đại dựa trên kernel dành cho thiết bị Android.</strong>
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

## Tổng quan

BunnySU là một giải pháp root cấp kernel được thiết kế cho các thiết bị Android.
Nó cung cấp khả năng quản lý quyền root mạnh mẽ trực tiếp từ lớp kernel, mang lại trải nghiệm sạch, hiệu quả và systemless cho người dùng nâng cao, nhà phát triển và người yêu thích Android.

Khác với các giải pháp root truyền thống chỉ hoạt động ở userspace, BunnySU tích hợp với kernel để mang lại khả năng kiểm soát sâu hơn, cách ly mạnh hơn và quản lý quyền linh hoạt hơn.

---

## Tính năng

* **Quyền root dựa trên kernel**
  Cung cấp hỗ trợ `su` trực tiếp thông qua Android kernel.

* **Quản lý quyền root**
  Kiểm soát ứng dụng nào có thể truy cập root bằng một hệ thống quyền rõ ràng và dễ quản lý.

* **Hỗ trợ module systemless**
  Hỗ trợ các thay đổi dựa trên module mà không cần sửa trực tiếp phân vùng system.

* **Điều khiển App Profile**
  Giới hạn, cách ly hoặc tùy chỉnh hành vi root cho từng ứng dụng riêng lẻ.

* **Hỗ trợ GKI**
  Được thiết kế cho các thiết bị Android hiện đại sử dụng kiến trúc Generic Kernel Image.

* **Khả năng tương thích nâng cao**
  Hỗ trợ điện thoại Android, WSA, ChromeOS và các môi trường Android dựa trên container.

---

## Tương thích

BunnySU chính thức hướng tới các thiết bị Android sử dụng **GKI 2.0** với kernel **5.10 hoặc mới hơn**.

Kernel cũ hơn cũng có thể hoạt động, nhưng thường cần tích hợp kernel thủ công và build riêng cho từng thiết bị.

### Kiến trúc được hỗ trợ

* `arm64-v8a`
* `x86_64`

> [!CAUTION]
> Một số thay đổi kernel gần đây có thể ảnh hưởng đến khả năng tương thích trên một số thiết bị `x86_64` và có thể gây lỗi boot hoặc kernel panic.
> Luôn kiểm tra thông tin riêng của thiết bị trước khi flash hoặc build.

---

## Cài đặt

Vui lòng đọc hướng dẫn cài đặt chính thức trước khi sử dụng BunnySU.

* [Installation Guide](https://kernelsu.org/guide/installation.html)
* [How to Build](https://kernelsu.org/guide/how-to-build.html)
* [Official Website](https://kernelsu.org/)

> [!WARNING]
> Root và chỉnh sửa kernel có thể gây lỗi boot, mất dữ liệu hoặc khiến thiết bị không ổn định nếu thực hiện sai.
> Hãy chắc chắn rằng bạn hiểu rủi ro và có bản sao lưu đầy đủ trước khi tiếp tục.

---

## Build

BunnySU có thể được tích hợp vào các Android kernel được hỗ trợ và build thủ công.

Để xem hướng dẫn build chi tiết, vui lòng tham khảo:

* [Build Documentation](https://kernelsu.org/guide/how-to-build.html)

Maintainer của thiết bị nên đảm bảo kernel source, defconfig và môi trường build đã được cấu hình đúng trước khi tích hợp.

---

## Hệ thống module

BunnySU hỗ trợ hệ thống module systemless dựa trên metamodules.

Module có thể được dùng để thay đổi hành vi hệ thống, thêm tính năng hoặc tùy chỉnh môi trường Android mà không cần sửa trực tiếp phân vùng system.

Tìm hiểu thêm:

* [Metamodule Documentation](https://kernelsu.org/guide/metamodule.html)

---

## App Profile

App Profile cho phép kiểm soát chi tiết quyền root và hành vi của ứng dụng.

Với App Profile, người dùng có thể giới hạn quyền root, tùy chỉnh đặc quyền và giảm việc lộ không cần thiết các khả năng root nhạy cảm.

Tìm hiểu thêm:

* [App Profile Documentation](https://kernelsu.org/guide/app-profile.html)

---

## Bảo mật

BunnySU hoạt động ở tầng thấp của hệ thống Android.
Vui lòng báo cáo vấn đề bảo mật một cách có trách nhiệm và tránh công khai lỗ hổng trước khi chúng được xem xét.

Thông tin liên quan đến bảo mật:

* [Security Policy](SECURITY.md)

---

## Giấy phép

BunnySU tuân theo cấu trúc giấy phép gốc của KernelSU:

* Các file trong thư mục `kernel` được cấp phép theo **GPL-2.0-only**.
* Các phần khác của dự án được cấp phép theo **GPL-3.0-or-later**, trừ khi có ghi chú khác.

Xem file [LICENSE](LICENSE) để biết thêm chi tiết.

---

## Credits

BunnySU được truyền cảm hứng từ và xây dựng dựa trên công sức của cộng đồng Android root và phát triển kernel.

Đặc biệt cảm ơn:

* [Kernel-Assisted Superuser](https://git.zx2c4.com/kernel-assisted-superuser/about/)
* [Magisk](https://github.com/topjohnwu/Magisk)
* [genuine](https://github.com/brevent/genuine/)
* [Diamorphine](https://github.com/m0nad/Diamorphine)

---

<p align="center">
  <strong>BunnySU</strong><br>
  Quyền root cấp kernel cho các thiết bị Android hiện đại.
</p>
