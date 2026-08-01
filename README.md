# 🧹 pac-clean

> Простая и безопасная CLI-утилита на Rust для поиска и интерактивной очистки осиротевших пакетов (orphans) в Arch Linux.

![Arch Linux](https://img.shields.io/badge/Arch%20Linux-1793D1?style=for-the-badge&logo=arch-linux&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/License-MIT-green.svg?style=for-the-badge)

## 📌 Особенности

* **Безопасность:** Не вызывает `pacman -Rns` вслепую, если в системе нет ненужных пакетов (избавляет от ошибки `no targets specified`).
* **Интерактивность:** Показывает полный список найденных пакетов и запрашивает подтверждение `[y/N]` перед вызовом `sudo`.
* **Высокая скорость:** Скомпилирована на нативном Rust без лишних сторонних зависимостей.

---

## 🛠️ Установка

### 1. Установка зависимостей

Перед сборкой убедись, что в системе установлены `git`, компилятор `rust` и инструментарий сборки:

```bash
sudo pacman -S --needed git rust base-devel
