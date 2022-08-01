use std::{fs, path::PathBuf};

use rocket::serde::json::Json;

use crate::Package;

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
        let path = entry.unwrap().path().join("package.toml");
        if !path.exists() {
            continue;
        }
        versions.push(toml::from_str(&fs::read_to_string(&path).unwrap()).unwrap());
    }
    // -----------

    Some(Json(Package {
        author: "unknown".to_string(),
        name: name.clone(),
        friendly_name: name.clone(),
        versions,
    }))
}
