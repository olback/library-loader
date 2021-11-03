use {
    crate::{consts, error::Result},
    reqwest,
    serde::Deserialize,
};

mod client;
pub use client::ClientKind;

#[derive(Deserialize)]
struct CargoToml {
    package: Package,
}

#[derive(Deserialize)]
struct Package {
    version: String,
}

#[derive(Debug)]
pub struct UpdateInfo<'l, 'u> {
    pub local: &'l str,
    pub remote: String,
    pub url: &'u str,
}

pub fn check<'l>(local_version: &'l str, kind: ClientKind) -> Result<Option<UpdateInfo>> {
    let url = format!(
        "https://raw.githubusercontent.com/olback/library-loader/master/{kind}/Cargo.toml",
        kind = kind
    );

    let remote_toml_str = reqwest::blocking::get(url)?.text()?;
    let remote_toml = toml::from_str::<CargoToml>(&remote_toml_str)?;

    if remote_toml.package.version == local_version {
        Ok(None)
    } else {
        Ok(Some(UpdateInfo {
            local: local_version,
            remote: remote_toml.package.version,
            url: consts::UPDATE_URL,
        }))
    }
}
