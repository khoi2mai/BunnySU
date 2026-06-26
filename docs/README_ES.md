# 🐰 BunnySU

<p align="center">
  <img src="https://raw.githubusercontent.com/khoi2mai/BunnySU/refs/heads/bunnysu-v3.2.4/docs/assets/bunnysu_logo.png" width="96" alt="BunnySU Logo">
</p>

<p align="center">
  <strong>Una solución root moderna basada en kernel para dispositivos Android.</strong>
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

## Descripción general

BunnySU es una solución root a nivel de kernel diseñada para dispositivos Android.
Proporciona una gestión potente del acceso root directamente desde la capa del kernel, ofreciendo una experiencia limpia, eficiente y systemless para usuarios avanzados, desarrolladores y entusiastas de Android.

A diferencia de las soluciones root tradicionales basadas únicamente en el espacio de usuario, BunnySU se integra con el kernel para ofrecer un control más profundo, un aislamiento más sólido y una gestión de permisos más flexible.

---

## Características

* **Acceso root basado en kernel**
  Proporciona soporte `su` directamente a través del kernel de Android.

* **Gestión de permisos root**
  Controla qué aplicaciones pueden acceder a root mediante un sistema de permisos claro y manejable.

* **Soporte para módulos systemless**
  Permite modificaciones basadas en módulos sin modificar directamente la partición del sistema.

* **Control de App Profile**
  Restringe, aísla o personaliza el comportamiento root para aplicaciones individuales.

* **Soporte para GKI**
  Diseñado para dispositivos Android modernos que utilizan la arquitectura Generic Kernel Image.

* **Compatibilidad avanzada**
  Compatible con teléfonos Android, WSA, ChromeOS y entornos Android basados en contenedores.

---

## Compatibilidad

BunnySU está orientado oficialmente a dispositivos Android que utilizan **GKI 2.0** con kernel **5.10 o superior**.

Los kernels más antiguos también pueden funcionar, pero normalmente requieren integración manual en el kernel y compilaciones específicas para cada dispositivo.

### Arquitecturas compatibles

* `arm64-v8a`
* `x86_64`

> [!CAUTION]
> Algunos cambios recientes del kernel pueden afectar la compatibilidad en ciertos dispositivos `x86_64` y podrían causar problemas de arranque o kernel panic.
> Revisa siempre la información específica de tu dispositivo antes de flashear o compilar.

---

## Instalación

Lee la guía oficial de instalación antes de usar BunnySU.

* [Guía de instalación](https://kernelsu.org/guide/installation.html)
* [Cómo compilar](https://kernelsu.org/guide/how-to-build.html)
* [Sitio web oficial](https://kernelsu.org/)

> [!WARNING]
> Rootear y modificar el kernel puede causar fallos de arranque, pérdida de datos o inestabilidad del dispositivo si se hace incorrectamente.
> Asegúrate de comprender los riesgos y de mantener una copia de seguridad completa antes de continuar.

---

## Compilación

BunnySU puede integrarse en kernels Android compatibles y compilarse manualmente.

Para instrucciones detalladas de compilación, consulta:

* [Documentación de compilación](https://kernelsu.org/guide/how-to-build.html)

Los mantenedores de dispositivos deben asegurarse de que el código fuente del kernel, el defconfig y el entorno de compilación estén correctamente configurados antes de la integración.

---

## Sistema de módulos

BunnySU admite un sistema de módulos systemless basado en metamódulos.

Los módulos pueden usarse para modificar el comportamiento del sistema, añadir funciones o personalizar el entorno Android sin cambiar directamente la partición del sistema.

Más información:

* [Documentación de metamódulos](https://kernelsu.org/guide/metamodule.html)

---

## App Profile

App Profile permite un control detallado sobre los permisos root y el comportamiento de las aplicaciones.

Con App Profile, los usuarios pueden limitar el acceso root, personalizar privilegios y reducir la exposición innecesaria de capacidades root sensibles.

Más información:

* [Documentación de App Profile](https://kernelsu.org/guide/app-profile.html)

---

## Seguridad

BunnySU opera a un nivel bajo del sistema Android.
Informa los problemas de seguridad de forma responsable y evita divulgar vulnerabilidades públicamente antes de que sean revisadas.

Para información relacionada con seguridad, consulta:

* [Política de seguridad](SECURITY.md)

---

## Licencia

BunnySU sigue la estructura de licencias original de KernelSU:

* Los archivos dentro del directorio `kernel` están licenciados bajo **GPL-2.0-only**.
* Las demás partes del proyecto están licenciadas bajo **GPL-3.0-or-later**, salvo que se indique lo contrario.

Consulta el archivo [LICENSE](LICENSE) para más detalles.

---

## Créditos

BunnySU está inspirado en el trabajo de la comunidad de desarrollo root y kernel de Android, y se construye sobre él.

Agradecimientos especiales a:

* [Kernel-Assisted Superuser](https://git.zx2c4.com/kernel-assisted-superuser/about/)
* [Magisk](https://github.com/topjohnwu/Magisk)
* [genuine](https://github.com/brevent/genuine/)
* [Diamorphine](https://github.com/m0nad/Diamorphine)

---

<p align="center">
  <strong>BunnySU</strong><br>
  Acceso root a nivel de kernel para dispositivos Android modernos.
</p>
