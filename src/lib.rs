pub fn version() -> &'static str {
    // Esta macro extrae la versión directamente de tu Cargo.toml
    env!("CARGO_PKG_VERSION")
}

pub fn info() {
    println!(
        "Vicode v{}: Validated Infrastructure-from-Code Core.",
        version()
    );
}
