use reqwest;
use toml;
use serde::Deserialize;
use super::error::LLResult;

pub struct UpdateInfo {
    pub local: String,
    pub remote: String
}

#[derive(Deserialize)]
struct CargoToml {
    package: Package
}

#[derive(Deserialize)]
struct Package {
    version: String
}

pub fn check(local_toml_str: &str, remote_toml_url: &str) -> LLResult<Option<UpdateInfo>> {

    let remote_toml_str = reqwest::get(remote_toml_url)?.text()?;

    let remote_toml: CargoToml = toml::from_str(remote_toml_str.as_str())?;
    let local_toml: CargoToml = toml::from_str(local_toml_str)?;

    if remote_toml.package.version != local_toml.package.version {

        Ok(Some(UpdateInfo {
            local: local_toml.package.version,
            remote: remote_toml.package.version
        }))

    } else {

        Ok(None)

    }

}
