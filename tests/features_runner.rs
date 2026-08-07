use cucumber::World;

pub mod steps;
pub mod e2e_steps;

#[derive(Debug, Default, World)]
pub struct WatnWorld;

#[tokio::main]
async fn main() {
    let spec_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("givn")
        .join("changes")
        .join("watn-cli")
        .join("specs");

    WatnWorld::cucumber()
        .fail_on_skipped()
        .run_and_exit(spec_dir)
        .await;
}
