use bevy_asset_packer::{AssetBundler, AssetBundlingOptions};
use std::{env::var, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    generate_icon();
    generate_assets()?;
    Ok(())
}

fn generate_assets() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=assets");
    let output = PathBuf::from(var("CARGO_MANIFEST_DIR")?).join("target").join("assets.pack");
    AssetBundler::new("assets", output, AssetBundlingOptions { compress: true, encryption_key: None }).build()?;

    Ok(())
}

fn generate_icon() {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/icon.ico");
        res.set("ProductName", "Artifact - THe Journey Unraveled");
        res.compile().expect("Failed to compile resource");
    }
}
