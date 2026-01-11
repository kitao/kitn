use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let pocketpy_src = out_dir.join("pocketpy");

    // 1. Clone pocketpy if not exists
    if !pocketpy_src.exists() {
        let status = Command::new("git")
            .args(&["clone", "--depth", "1", "https://github.com/pocketpy/pocketpy.git", "pocketpy"])
            .current_dir(&out_dir)
            .status()
            .expect("Failed to execute git clone");

        if !status.success() {
            panic!("git clone failed");
        }
    }

    // 1.5. Patch amalgamate.py to use python3
    let amalgamate_path = pocketpy_src.join("amalgamate.py");
    let content = std::fs::read_to_string(&amalgamate_path).expect("Failed to read amalgamate.py");
    let patched_content = content.replace("python prebuild.py", "python3 prebuild.py");
    std::fs::write(&amalgamate_path, patched_content).expect("Failed to write patched amalgamate.py");

    // 2. Run amalgamate.py
    // Note: Python3 is required.
    let status = Command::new("python3")
        .arg("amalgamate.py")
        .current_dir(&pocketpy_src)
        .status()
        .expect("Failed to execute python3 amalgamate.py");

    if !status.success() {
        panic!("amalgamate.py failed");
    }

    // 3. Compiling
    // The amalgamation script usually outputs to `amalgamated/pocketpy.c` or similar.
    // We will check both root and amalgamated subfolder just in case, or default to one.
    // Based on common patterns in pocketpy, it is likely `amalgamated.c` or `pocketpy.c` in root or `amalgamated` dir.
    // Let's assume `amalgamated/pocketpy.cpp` or `.c`. Since it is C++, likelihood of .cpp is high, but the task said "pocketpy-sys", often C binding?
    // PocketPy describes itself as "Single file (pocketpy.h + pocketpy.cpp)".
    // So we look for pocketpy.cpp.


    let amalgamated_dir = pocketpy_src.join("amalgamated");
    let source_path_cpp = amalgamated_dir.join("pocketpy.cpp");
    let source_path_c = amalgamated_dir.join("pocketpy.c");

    let source_path = if source_path_cpp.exists() {
        source_path_cpp
    } else if source_path_c.exists() {
        source_path_c
    } else {
        pocketpy_src.join("pocketpy.cpp")
    };

    if !source_path.exists() {
        let entries: Vec<_> = std::fs::read_dir(&amalgamated_dir)
            .map(|dirs| dirs.map(|e| e.map(|e| e.path())).collect())
            .unwrap_or_default();
        panic!("Could not find pocketpy.cpp or pocketpy.c in {:?}. Contents: {:?}", amalgamated_dir, entries);
    }

    let mut build = cc::Build::new();
    build.file(&source_path);

    if source_path.extension().map_or(false, |ext| ext == "cpp") {
        build.cpp(true).std("c++17");
    } else {
        // Assuming C11 for .c file if not C++
        build.std("c11").flag("-w");
    }

    build.compile("pocketpy");

    // 4. Generate Bindings
    // Header file should be pocketpy.h
    let header_path = source_path.with_file_name("pocketpy.h");

    println!("cargo:rerun-if-changed=build.rs");
    // Also rerun if header changes
    println!("cargo:rerun-if-changed={}", header_path.display());

    let bindings = bindgen::Builder::default()
        .header(header_path.to_str().unwrap())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .layout_tests(false) // Sometimes layout tests fail on cross-platform or complex C++ structs
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
