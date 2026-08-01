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

### Требования
* Arch Linux (или любой дистрибутив на его базе: Manjaro, EndeavourOS)
* Установленный `pacman` и `rust` / `cargo`

### Сборка из исходников

```bash
# 1. Клонировать репозиторий
git clone [https://github.com/sqeezy-53/pac-clean.git](https://github.com/sqeezy-53/pac-clean.git)
cd pac-clean

# 2. Собрать релизную версию
cargo build --release

# 3. Скопировать бинарник в локальный PATH
mkdir -p ~/.local/bin
cp target/release/pac-clean ~/.local/bin/
