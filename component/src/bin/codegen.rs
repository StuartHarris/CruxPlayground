use std::path::Path;

use anyhow::Result;
use crux_core::type_generation::facet::{Config, ExternalPackage, PackageLocation, TypeRegistry};
use component::{Component};

fn main() -> Result<()> {
    let out_dir = Path::new(".");

    println!("Generating code for Serde");
    serde(&out_dir)?;

    println!("Generating code for ComponentCrux");
    component_crux(&out_dir)?;

    Ok(())
}

fn component_crux(out_dir: &Path) -> Result<()> {
    let typegen_app = TypeRegistry::new().register_app::<Component>().build();
    typegen_app.swift(
        &Config::builder("ComponentCrux", out_dir.join("generated"))
            .reference(ExternalPackage {
                for_namespace: "serde".to_string(),
                location: PackageLocation::Path(
                    "../Serde".to_string(),
                ),
                module_name: None,
                version: None,
            })
            .add_extensions()
            .build(),
    )?;

    Ok(())
}

fn serde(out_dir: &Path) -> Result<()> {
    let typegen_serde = TypeRegistry::new().build();

    typegen_serde.swift(
        &Config::builder("Serde", out_dir.join("generated"))
            .add_runtimes()
            .build(),
    )?;

    Ok(())
}

