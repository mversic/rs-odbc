fn main() {
    if std::env::var("CARGO_FEATURE_STATIC").is_ok() {
        let odbc_path = std::env::var("STATIC_ODBC_PATH").expect("Path to static ODBC lib required");

        if cfg!(target_os = "windows") {
            panic!("static linking is not supported for windows target");
        }

        println!("cargo:rustc-link-search=native={}", odbc_path);
        println!("cargo:rerun-if-env-changed=STATIC_ODBC_PATH");
        println!("cargo:rustc-link-lib=static=odbc");
    }
}
