#  🐰 BunnySU

<p align="center">
  <img src="https://raw.githubusercontent.com/khoi2mai/BunnySU/refs/heads/bunnysu/docs/assets/bunnysu_logo.png" width="96" alt="BunnySU Logo">
</p>

<p align="center">
  <strong>Uma solução root moderna baseada em kernel para dispositivos Android.</strong>
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

## Visão geral

BunnySU é uma solução root em nível de kernel projetada para dispositivos Android.
Ela oferece gerenciamento poderoso de acesso root diretamente a partir da camada do kernel, proporcionando uma experiência limpa, eficiente e systemless para usuários avançados, desenvolvedores e entusiastas do Android.

Diferente das soluções root tradicionais baseadas apenas em userspace, o BunnySU se integra ao kernel para oferecer controle mais profundo, isolamento mais forte e gerenciamento de permissões mais flexível.

---

## Recursos

* **Acesso root baseado em kernel**
  Fornece suporte a `su` diretamente pelo kernel do Android.

* **Gerenciamento de permissões root**
  Controle quais apps podem acessar root com um sistema de permissões claro e gerenciável.

* **Suporte a módulos systemless**
  Suporta modificações baseadas em módulos sem alterar diretamente a partição do sistema.

* **Controle de App Profile**
  Restrinja, isole ou personalize o comportamento root para apps individuais.

* **Suporte a GKI**
  Projetado para dispositivos Android modernos que usam a arquitetura Generic Kernel Image.

* **Compatibilidade avançada**
  Suporta telefones Android, WSA, ChromeOS e ambientes Android baseados em contêiner.

---

## Compatibilidade

O BunnySU tem como alvo oficial dispositivos Android que usam **GKI 2.0** com kernel **5.10 ou mais recente**.

Kernels mais antigos também podem funcionar, mas geralmente exigem integração manual ao kernel e builds específicos para o dispositivo.

### Arquiteturas suportadas

* `arm64-v8a`
* `x86_64`

> [!CAUTION]
> Algumas alterações recentes no kernel podem afetar a compatibilidade em certos dispositivos `x86_64` e causar problemas de boot ou kernel panic.
> Sempre verifique as informações específicas do dispositivo antes de fazer flash ou build.

---

## Instalação

Leia o guia oficial de instalação antes de usar o BunnySU.

* [Installation Guide](https://kernelsu.org/guide/installation.html)
* [How to Build](https://kernelsu.org/guide/how-to-build.html)
* [Official Website](https://kernelsu.org/)

> [!WARNING]
> Fazer root e modificar o kernel pode causar falha de boot, perda de dados ou instabilidade do dispositivo se feito incorretamente.
> Certifique-se de entender os riscos e mantenha um backup completo antes de continuar.

---

## Build

O BunnySU pode ser integrado a kernels Android compatíveis e compilado manualmente.

Para instruções detalhadas de build, consulte:

* [Build Documentation](https://kernelsu.org/guide/how-to-build.html)

Mantenedores de dispositivos devem garantir que o código-fonte do kernel, o defconfig e o ambiente de build estejam configurados corretamente antes da integração.

---

## Sistema de módulos

O BunnySU suporta um sistema de módulos systemless baseado em metamodules.

Módulos podem ser usados para modificar o comportamento do sistema, adicionar recursos ou personalizar o ambiente Android sem alterar diretamente a partição do sistema.

Saiba mais:

* [Metamodule Documentation](https://kernelsu.org/guide/metamodule.html)

---

## App Profile

App Profile permite controle detalhado sobre permissões root e comportamento dos apps.

Com App Profile, usuários podem limitar o acesso root, personalizar privilégios e reduzir a exposição desnecessária de capacidades root sensíveis.

Saiba mais:

* [App Profile Documentation](https://kernelsu.org/guide/app-profile.html)

---

## Segurança

BunnySU opera em um nível baixo do sistema Android.
Relate problemas de segurança de forma responsável e evite divulgar vulnerabilidades publicamente antes que sejam analisadas.

Para informações relacionadas à segurança, consulte:

* [Security Policy](SECURITY.md)

---

## Licença

BunnySU segue a estrutura original de licenciamento do KernelSU:

* Arquivos no diretório `kernel` são licenciados sob **GPL-2.0-only**.
* Outras partes do projeto são licenciadas sob **GPL-3.0-or-later**, salvo indicação em contrário.

Veja o arquivo [LICENSE](LICENSE) para mais detalhes.

---

## Créditos

BunnySU é inspirado e construído sobre o trabalho da comunidade de root e desenvolvimento de kernel do Android.

Agradecimentos especiais a:

* [Kernel-Assisted Superuser](https://git.zx2c4.com/kernel-assisted-superuser/about/)
* [Magisk](https://github.com/topjohnwu/Magisk)
* [genuine](https://github.com/brevent/genuine/)
* [Diamorphine](https://github.com/m0nad/Diamorphine)

---

<p align="center">
  <strong>BunnySU</strong><br>
  Acesso root em nível de kernel para dispositivos Android modernos.
</p>
