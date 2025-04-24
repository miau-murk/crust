#!/bin/bash

# Устанавливаем Rust
curl https://sh.rustup.rs -sSf | bash -s -- -y
source "$HOME/.cargo/env"

# Устанавливаем зависимости (если нужны)
sudo apt install -y glibc-source  # или другие зависимости, если требуются

# Клонируем репозиторий
git clone https://github.com/miau-murk/crust.git
cd crust || exit 1  # Выходим, если переход не удался

# Копируем test.cpp в нужное место (если файл существует)
mkdir -p build_c  # Создаём папку, если её нет
cp "../test.cpp" "build_c/logpc.cpp"

# Собираем проект
cargo build --verbose --release