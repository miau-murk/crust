#!/bin/bash

# ������������� Rust
curl https://sh.rustup.rs -sSf | bash -s -- -y
source "$HOME/.cargo/env"

# ������������� ����������� (���� �����)
sudo apt install -y glibc-source  # ��� ������ �����������, ���� ���������

# ��������� �����������
git clone https://github.com/miau-murk/crust.git
cd crust || exit 1  # �������, ���� ������� �� ������

# �������� test.cpp � ������ ����� (���� ���� ����������)
# mkdir -p build_c  # ������ �����, ���� � ���
# cp "../test.cpp" "build_c/logpc.cpp"

# �������� ������
cargo build --verbose --release