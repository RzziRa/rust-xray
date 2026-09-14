#[tokio::main]
async fn main() {
    if let Err(err) = rust_xray::app::main_entry().await {
        rust_xray::eprintln_bootstrap!("fatal: {err:?}");
        std::process::exit(1);
    }
}
