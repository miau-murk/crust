use std::process::Command;
use std::env;
use std::path::Path;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let out_dir = env::var("OUT_DIR").map_err(|e| {
        io::Error::new(io::ErrorKind::Other, format!("Failed to get OUT_DIR: {}", e))
    })?;
    let out_path = Path::new(&out_dir);

    // 1. Компиляция C++ файла
    let cpp_file = "build_c/logpc.cpp";
    let obj_file = out_path.join("logpc.o");

    if !Path::new(cpp_file).exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("C++ source file {} not found", cpp_file)
        ));
    }

    let compile_status = Command::new("g++")
        .args(&[
            cpp_file,
            "-c",           // Только компиляция без линковки
            "-fPIC",        // Позиционно-независимый код
            "-o",
            obj_file.to_str().unwrap(),
            "-Wall",        // Включить все предупреждения
            "-std=c++14"    // Использовать C++14 стандарт
        ])
        .status()?;

    if !compile_status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to compile C++ code"
        ));
    }

    // 2. Создание статической библиотеки
    let lib_file = out_path.join("liblogpc.a");
    let ar_status = Command::new("ar")
        .args(&["crus", lib_file.to_str().unwrap(), obj_file.to_str().unwrap()])
        .status()?;

    if !ar_status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to create static library"
        ));
    }

    // 3. Настройка линковки
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=logpc");
    println!("cargo:rustc-link-lib=dylib=stdc++");  // Явная линковка с C++ stdlib
    println!("cargo:rerun-if-changed={}", cpp_file);

    Ok(())
}