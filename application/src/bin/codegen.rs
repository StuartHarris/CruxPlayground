use std::path::Path;

use anyhow::Result;
use application::Application;
use crux_core::type_generation::facet::{Config, ExternalPackage, PackageLocation, TypeRegistry};

fn main() -> Result<()> {
    let out_dir = Path::new(".");

    println!("Generating code for ApplicationCrux");
    application_crux(&out_dir)?;

    Ok(())
}

fn application_crux(out_dir: &Path) -> Result<()> {
    let typegen_app = TypeRegistry::new().register_app::<Application>().build();
    typegen_app.swift(
        &Config::builder("ApplicationCrux", out_dir.join("generated"))
            .reference(ExternalPackage {
                for_namespace: "serde".to_string(),
                location: PackageLocation::Path("../Serde".to_string()),
                module_name: None,
                version: None,
            })
            .reference(ExternalPackage {
                for_namespace: "component_crux".to_string(),
                location: PackageLocation::Path("../ComponentCrux".to_string()),
                module_name: None,
                version: None,
            })
            .add_extensions()
            .build(),
    )?;

    Ok(())
}
