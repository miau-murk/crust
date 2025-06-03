# crust

```
#!/bin/bash

# download cargo Rust compiler
curl https://sh.rustup.rs -sSf | bash -s -- -y
source "$HOME/.cargo/env"

# cloning project repository
git clone https://github.com/miau-murk/crust.git
cd crust || exit 1

# building project
cargo build --verbose --release
```
