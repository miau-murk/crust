# crust

This project enables sampling from distributions with a custom .cpp density function using [nuts-rs](https://github.com/pymc-devs/nuts-rs) on Linux systems.

## Installation

First, install the Rust Cargo compiler:

```bash
# Download and install Rust compiler
curl https://sh.rustup.rs -sSf | bash -s -- -y
source "$HOME/.cargo/env"
```

After successful installation, clone this repository to your local folder:

```bash
# cloning project repository
git clone https://github.com/miau-murk/crust.git
cd crust || exit 1
```

The core of the program is located in `src/lib.rs`, which facilitates the interaction between C++ functions and the functionality of [nuts-rs](https://github.com/pymc-devs/nuts-rs), as well as logging statistics. The Cargo.toml file is required for Rust module dependencies.

The `build_c` folder contains functions implementing a multivariate normal distribution with zero mean. The key script is `logpc.cpp`, which contains the function passed to Rust modules. For a test run, we recommend first building the program with the default density function:

```bash
# building project
cargo build --verbose --release
```

After building the program, you will find the dynamic library `libnuts_impl.so` in the `target/release` folder. An example of how to link the library to C++ code can be found in the files `nuts_impl.h` and `test.cpp`. The function takes the number of samples and the parameter space dimension as input. To run the program, use the following commands:

```bash
gcc test.cpp -o test -L./target/release -lnuts_impl
LD_LIBRARY_PATH=./target/release ./test
```

After executing these commands, a file named `samples.log` will be created in the project folder, containing the generated samples and the mean for each coordinate.

To sample from custom functions, you need to create a function named `logpc` in the file `build_c/logpc.cpp`, with similar structure to the original one. All files imported into your `logpc.cpp` script must either be located in the `build_c` folder or be provided as static libraries.
