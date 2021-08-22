fn main() {
    if std::env::var("CARGO_FEATURE_STATIC").is_ok() {
        if cfg!(target_os = "windows") {
            panic!("static linking is not supported for Windows");
        }

        let odbc_path = std::env::var("RS_ODBC_LINK_SEARCH")
            .expect("Path to native static libraries is required");

        println!("cargo:rerun-if-env-changed=RS_ODBC_LINK_SEARCH");
        println!("cargo:rustc-link-search=native={}", odbc_path);

        println!("cargo:rustc-link-lib=static=odbc");
        println!("cargo:rustc-link-lib=static=ltdl");
    }
}
