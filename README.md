# pac-clean

> Простая и безопасная CLI-утилита на Rust для поиска и интерактивной очистки осиротевших пакетов в Arch Linux.

## Особенности

* **Безопасность:** Не вызывает `pacman -Rns` вслепую, если в системе нет ненужных пакетов (избавляет от ошибки `no targets specified`).
* **Интерактивность:** Показывает полный список найденных пакетов и запрашивает подтверждение `[y/N]` перед вызовом `sudo`.
* **Высокая скорость:** Скомпилирована на нативном Rust без лишних сторонних зависимостей.

---

## Установка

```bash
sudo pacman -S --needed git rust base-devel && \
git clone https://github.com/sqeezy-53/pac-clean.git && \
cd pac-clean && \
cargo build --release && \
mkdir -p ~/.local/bin && \
cp target/release/pac-clean ~/.local/bin/ && \
echo && \
echo "pac-clean успешно установлен!" && \
echo "Если ~/.local/bin отсутствует в PATH, добавьте в ~/.bashrc или ~/.zshrc:" && \
echo 'export PATH="$HOME/.local/bin:$PATH"'
```

---

## Использование

```bash
pac-clean
```
