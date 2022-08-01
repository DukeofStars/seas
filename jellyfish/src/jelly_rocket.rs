use std::{fs, path::PathBuf};

use guid_create::GUID;
use rocket::serde::json::Json;

use crate::{Package, PackageVersion};

#[get("/get/<name>")]
pub fn get_package(name: String) -> Option<Json<Package>> {
    let path = PathBuf::new().join("dist").join(&name);
    if !path.exists() {
        return None;
    }

    let mut versions = vec![];

    // Debug only. Should switch to using database.
    let entries = fs::read_dir(&path).unwrap();
    for entry in entries {
        let path = entry.unwrap().path();
        // Split it to get <version>-<guid>. eg. 1.0.0-12345678-12345678-12345678-12345678
        let file_sections = path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .split_once("-")
            .unwrap();
        println!("{}-{}", file_sections.0, file_sections.1);
        let version = file_sections.0;
        let guid = GUID::parse(file_sections.1).unwrap();
        versions.push(PackageVersion {
            id: guid,
            name: name.clone(),
            version: version.to_string(),
            required: vec![],
            dependencies: vec![],
        });
    }
    // -----------

    Some(Json(Package {
        author: "unknown".to_string(),
        name: name.clone(),
        friendly_name: name.clone(),
        versions,
    }))
}
