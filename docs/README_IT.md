#  🐰 BunnySU

<p align="center">
  <img src="https://raw.githubusercontent.com/khoi2mai/BunnySU/refs/heads/bunnysu/docs/assets/bunnysu_logo.png" width="96" alt="BunnySU Logo">
</p>

<p align="center">
  <strong>Una moderna soluzione root basata sul kernel per dispositivi Android.</strong>
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
  <a href="README.md">Tiếng Việt</a> |
  <a href="README_ES.md">Español</a> |
  <a href="README_CN.md">简体中文</a> |
  <a href="README_TW.md">繁體中文</a> |
  <a href="README_JP.md">日本語</a> |
  <a href="README_KR.md">한국어</a> |
  <a href="README_PL.md">Polski</a> |
  <a href="README_PT-BR.md">Português</a> |
  <a href="README_TR.md">Türkçe</a> |
  <a href="README_RU.md">Русский</a> |
  <a href="README_EN.md">English</a> |
  <a href="README_ID.md">Indonesia</a> |
  <a href="README_IN.md">हिन्दी</a> |
  <a href="README_IT.md">Italiano</a> |
  <a href="README_IW.md">עברית</a>
</p>

---

## Panoramica

BunnySU è una soluzione root a livello kernel progettata per dispositivi Android.
Fornisce una gestione potente dell'accesso root direttamente dal livello kernel, offrendo un'esperienza pulita, efficiente e systemless per utenti avanzati, sviluppatori e appassionati Android.

A differenza delle soluzioni root tradizionali basate solo sullo userspace, BunnySU si integra con il kernel per offrire un controllo più profondo, un isolamento più forte e una gestione dei permessi più flessibile.

---

## Funzionalità

* **Accesso root basato sul kernel**
  Fornisce il supporto `su` direttamente tramite il kernel Android.

* **Gestione dei permessi root**
  Controlla quali app possono accedere al root tramite un sistema di permessi chiaro e gestibile.

* **Supporto ai moduli systemless**
  Supporta modifiche basate su moduli senza modificare direttamente la partizione di sistema.

* **Controllo App Profile**
  Limita, isola o personalizza il comportamento root per le singole app.

* **Supporto GKI**
  Progettato per dispositivi Android moderni che utilizzano l'architettura Generic Kernel Image.

* **Compatibilità avanzata**
  Supporta telefoni Android, WSA, ChromeOS e ambienti Android basati su container.

---

## Compatibilità

BunnySU è pensato ufficialmente per dispositivi Android che utilizzano **GKI 2.0** con kernel **5.10 o più recente**.

Anche i kernel più vecchi potrebbero funzionare, ma di solito richiedono integrazione manuale nel kernel e build specifiche per il dispositivo.

### Architetture supportate

* `arm64-v8a`
* `x86_64`

> [!CAUTION]
> Alcune modifiche recenti al kernel potrebbero influire sulla compatibilità di alcuni dispositivi `x86_64` e causare problemi di avvio o kernel panic.
> Controlla sempre le informazioni specifiche del tuo dispositivo prima di flashare o compilare.

---

## Installazione

Leggi la guida ufficiale all'installazione prima di usare BunnySU.

* [Installation Guide](https://kernelsu.org/guide/installation.html)
* [How to Build](https://kernelsu.org/guide/how-to-build.html)
* [Official Website](https://kernelsu.org/)

> [!WARNING]
> Il root e la modifica del kernel possono causare mancato avvio, perdita di dati o instabilità del dispositivo se eseguiti in modo errato.
> Assicurati di comprendere i rischi e conserva un backup completo prima di procedere.

---

## Compilazione

BunnySU può essere integrato nei kernel Android supportati e compilato manualmente.

Per istruzioni dettagliate sulla compilazione, consulta:

* [Build Documentation](https://kernelsu.org/guide/how-to-build.html)

I maintainer dei dispositivi devono assicurarsi che il sorgente del kernel, il defconfig e l'ambiente di build siano configurati correttamente prima dell'integrazione.

---

## Sistema di moduli

BunnySU supporta un sistema di moduli systemless basato su metamodules.

I moduli possono essere usati per modificare il comportamento del sistema, aggiungere funzionalità o personalizzare l'ambiente Android senza modificare direttamente la partizione di sistema.

Scopri di più:

* [Metamodule Documentation](https://kernelsu.org/guide/metamodule.html)

---

## App Profile

App Profile consente un controllo granulare sui permessi root e sul comportamento delle app.

Con App Profile, gli utenti possono limitare l'accesso root, personalizzare i privilegi e ridurre l'esposizione non necessaria di funzionalità root sensibili.

Scopri di più:

* [App Profile Documentation](https://kernelsu.org/guide/app-profile.html)

---

## Sicurezza

BunnySU opera a un livello basso del sistema Android.
Segnala responsabilmente i problemi di sicurezza ed evita di divulgare pubblicamente le vulnerabilità prima che vengano esaminate.

Per informazioni relative alla sicurezza, consulta:

* [Security Policy](SECURITY.md)

---

## Licenza

BunnySU segue la struttura di licenza originale di KernelSU:

* I file nella directory `kernel` sono concessi in licenza con **GPL-2.0-only**.
* Le altre parti del progetto sono concesse in licenza con **GPL-3.0-or-later**, salvo diversa indicazione.

Consulta il file [LICENSE](LICENSE) per maggiori dettagli.

---

## Crediti

BunnySU è ispirato al lavoro della community Android root e kernel development ed è costruito su tale lavoro.

Ringraziamenti speciali a:

* [Kernel-Assisted Superuser](https://git.zx2c4.com/kernel-assisted-superuser/about/)
* [Magisk](https://github.com/topjohnwu/Magisk)
* [genuine](https://github.com/brevent/genuine/)
* [Diamorphine](https://github.com/m0nad/Diamorphine)

---

<p align="center">
  <strong>BunnySU</strong><br>
  Accesso root a livello kernel per dispositivi Android moderni.
</p>
