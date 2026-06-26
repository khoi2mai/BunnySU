#  🐰 BunnySU

<p align="center">
  <img src="https://raw.githubusercontent.com/khoi2mai/BunnySU/refs/heads/bunnysu/docs/assets/bunnysu_logo.png" width="96" alt="BunnySU Logo">
</p>

<p align="center">
  <strong>Nowoczesne rozwiązanie root oparte na kernelu dla urządzeń Android.</strong>
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

## Przegląd

BunnySU to rozwiązanie root na poziomie kernela zaprojektowane dla urządzeń Android.
Zapewnia zaawansowane zarządzanie dostępem root bezpośrednio z warstwy kernela, oferując czyste, wydajne i systemless doświadczenie dla zaawansowanych użytkowników, deweloperów i entuzjastów Androida.

W przeciwieństwie do tradycyjnych rozwiązań root działających wyłącznie w userspace, BunnySU integruje się z kernelem, aby zapewnić głębszą kontrolę, mocniejszą izolację i bardziej elastyczne zarządzanie uprawnieniami.

---

## Funkcje

* **Dostęp root oparty na kernelu**
  Zapewnia obsługę `su` bezpośrednio przez kernel Androida.

* **Zarządzanie uprawnieniami root**
  Kontroluj, które aplikacje mogą uzyskać dostęp root, za pomocą przejrzystego i łatwego w zarządzaniu systemu uprawnień.

* **Obsługa modułów systemless**
  Obsługuje modyfikacje oparte na modułach bez bezpośredniej zmiany partycji systemowej.

* **Kontrola App Profile**
  Ograniczaj, izoluj lub dostosowuj zachowanie root dla poszczególnych aplikacji.

* **Obsługa GKI**
  Zaprojektowany dla nowoczesnych urządzeń Android korzystających z architektury Generic Kernel Image.

* **Zaawansowana kompatybilność**
  Obsługuje telefony Android, WSA, ChromeOS oraz środowiska Android oparte na kontenerach.

---

## Kompatybilność

BunnySU oficjalnie celuje w urządzenia Android używające **GKI 2.0** z kernelem **5.10 lub nowszym**.

Starsze kernele również mogą działać, ale zwykle wymagają ręcznej integracji z kernelem i buildów specyficznych dla urządzenia.

### Obsługiwane architektury

* `arm64-v8a`
* `x86_64`

> [!CAUTION]
> Niektóre ostatnie zmiany w kernelu mogą wpływać na kompatybilność na wybranych urządzeniach `x86_64` i powodować problemy z uruchamianiem lub kernel panic.
> Zawsze sprawdzaj informacje specyficzne dla urządzenia przed flashowaniem lub budowaniem.

---

## Instalacja

Przed użyciem BunnySU przeczytaj oficjalny przewodnik instalacji.

* [Installation Guide](https://kernelsu.org/guide/installation.html)
* [How to Build](https://kernelsu.org/guide/how-to-build.html)
* [Official Website](https://kernelsu.org/)

> [!WARNING]
> Rootowanie i modyfikowanie kernela może spowodować brak możliwości uruchomienia, utratę danych lub niestabilność urządzenia, jeśli zostanie wykonane nieprawidłowo.
> Upewnij się, że rozumiesz ryzyko i wykonaj pełną kopię zapasową przed kontynuacją.

---

## Budowanie

BunnySU można zintegrować z obsługiwanymi kernelami Androida i zbudować ręcznie.

Szczegółowe instrukcje budowania znajdziesz tutaj:

* [Build Documentation](https://kernelsu.org/guide/how-to-build.html)

Maintainerzy urządzeń powinni upewnić się, że źródła kernela, defconfig i środowisko build są poprawnie skonfigurowane przed integracją.

---

## System modułów

BunnySU obsługuje system modułów systemless oparty na metamodules.

Moduły mogą być używane do modyfikowania zachowania systemu, dodawania funkcji lub dostosowywania środowiska Android bez bezpośredniej zmiany partycji systemowej.

Dowiedz się więcej:

* [Metamodule Documentation](https://kernelsu.org/guide/metamodule.html)

---

## App Profile

App Profile umożliwia szczegółową kontrolę nad uprawnieniami root i zachowaniem aplikacji.

Dzięki App Profile użytkownicy mogą ograniczać dostęp root, dostosowywać uprawnienia i zmniejszać niepotrzebne narażenie wrażliwych możliwości root.

Dowiedz się więcej:

* [App Profile Documentation](https://kernelsu.org/guide/app-profile.html)

---

## Bezpieczeństwo

BunnySU działa na niskim poziomie systemu Android.
Zgłaszaj problemy bezpieczeństwa odpowiedzialnie i unikaj publicznego ujawniania podatności przed ich przejrzeniem.

Informacje związane z bezpieczeństwem:

* [Security Policy](SECURITY.md)

---

## Licencja

BunnySU zachowuje oryginalną strukturę licencjonowania KernelSU:

* Pliki w katalogu `kernel` są licencjonowane na **GPL-2.0-only**.
* Pozostałe części projektu są licencjonowane na **GPL-3.0-or-later**, chyba że wskazano inaczej.

Więcej informacji znajdziesz w pliku [LICENSE](LICENSE).

---

## Podziękowania

BunnySU jest inspirowany pracą społeczności Android root i kernel development oraz na niej zbudowany.

Specjalne podziękowania dla:

* [Kernel-Assisted Superuser](https://git.zx2c4.com/kernel-assisted-superuser/about/)
* [Magisk](https://github.com/topjohnwu/Magisk)
* [genuine](https://github.com/brevent/genuine/)
* [Diamorphine](https://github.com/m0nad/Diamorphine)

---

<p align="center">
  <strong>BunnySU</strong><br>
  Dostęp root na poziomie kernela dla nowoczesnych urządzeń Android.
</p>
