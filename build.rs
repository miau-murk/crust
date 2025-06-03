use std::process::Command;
use std::env;
use std::path::{Path, PathBuf};
use std::io::{self, Write};
use std::fs;

fn main() -> io::Result<()> {
    let out_dir = env::var("OUT_DIR").map_err(|e| {
        io::Error::new(io::ErrorKind::Other, format!("Failed to get OUT_DIR: {}", e))
    })?;
    let out_path = Path::new(&out_dir);
    let cpp_dir = "build_c"; // Директория с C++ файлами

    // 1. We find all the C++ files in the directory
    let cpp_files = find_cpp_files(cpp_dir)?;
    if cpp_files.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("No C++ source files found in {}", cpp_dir)
        ));
    }

    // 2. Compile each C++ file into an object file
    let mut obj_files = Vec::new();
    for cpp_file in &cpp_files {
        let obj_file = out_path.join(
            Path::new(cpp_file)
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
        ).with_extension("o");

        compile_cpp_file(cpp_file, &obj_file)?;
        obj_files.push(obj_file);
    }

    // 3. Creating a static library from all the object files
    let lib_file = out_path.join("liblogpc.a");
    create_static_library(&obj_files, &lib_file)?;

    // 4. Linking settings
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=logpc");
    println!("cargo:rustc-link-lib=dylib=stdc++");  // Explicit linking with C++ stdlib


    Ok(())
}

/// Finds all .cpp files in the specified directory
fn find_cpp_files(dir: &str) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("cpp") {
            files.push(path);
        }
    }
    Ok(files)
}

/// Compiles a single C++ file into an object file
fn compile_cpp_file(cpp_file: &Path, obj_file: &Path) -> io::Result<()> {
    let status = Command::new("g++")
        .args(&[
            cpp_file.to_str().unwrap(),
            "-c",
            "-fPIC",
            "-o",
            obj_file.to_str().unwrap(),
            "-Wall",
            "-Wextra",     
            "-std=c++11",
            "-O2",
        ])
        .status()?;

    if !status.success() {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to compile {}", cpp_file.display())
        ))
    } else {
        Ok(())
    }
}

/// Creates a static library from object files
fn create_static_library(obj_files: &[PathBuf], lib_file: &Path) -> io::Result<()> {
    let mut args = vec!["crus", lib_file.to_str().unwrap()];
    args.extend(obj_files.iter().map(|f| f.to_str().unwrap()));

    let status = Command::new("ar")
        .args(&args)
        .status()?;

    if !status.success() {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to create static library {}", lib_file.display())
        ))
    } else {
        Ok(())
    }
}