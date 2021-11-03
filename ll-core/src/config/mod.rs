use {
    crate::{
        consts::LL_CONFIG,
        error::{Error, Result},
        format::{self, ECAD},
    },
    profile::Profile,
    serde::{Deserialize, Serialize},
    std::{collections::HashMap, fs, path::PathBuf},
};

pub mod profile;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Format {
    pub format: ECAD,
    pub output_path: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Settings {
    pub watch_path: String,
    pub recursive: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(skip)]
    _self_path: Option<PathBuf>,
    pub settings: Settings,
    #[serde(default)]
    pub formats: HashMap<String, Format>,
    pub profile: Profile,
}

impl Config {
    pub fn read(path: Option<PathBuf>) -> Result<Self> {
        let p = Self::path(path)?.canonicalize()?;
        let mut s = toml::from_str::<Self>(&fs::read_to_string(&p)?)?;
        s._self_path = Some(p);
        Ok(s)
    }

    pub fn save(&self, path: Option<PathBuf>) -> Result<()> {
        let p = Self::path(path.or(self._self_path.clone()))?;
        let toml_str = toml::to_string_pretty(self)?;
        fs::write(p, toml_str)?;
        Ok(())
    }

    pub(crate) fn formats(&self) -> Result<Vec<format::Format>> {
        let mut formats_vec = Vec::with_capacity(self.formats.len());
        for (_, f) in &self.formats {
            formats_vec.push(format::Format::from_ecad(
                f.format.clone(),
                PathBuf::from(shellexpand::full(&f.output_path)?.as_ref()),
            ))
        }
        Ok(formats_vec)
    }

    fn path(path: Option<PathBuf>) -> Result<PathBuf> {
        path.or(Self::default_path())
            .ok_or(Error::Other("Could not find config dir".into()))
    }

    pub fn default_path() -> Option<PathBuf> {
        dirs::config_dir().map(|dir| dir.join(LL_CONFIG))
    }

    pub fn get_path() -> Result<Option<PathBuf>> {
        let local = PathBuf::from(LL_CONFIG);
        let local_exists = local.exists();
        let global = Self::default_path();
        let global_exists = global.as_ref().map(|p| p.exists());

        if local_exists {
            Ok(Some(local.canonicalize()?))
        } else if global_exists == Some(true) {
            Ok(Some(global.unwrap()))
        } else {
            Ok(None)
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            _self_path: None,
            settings: Settings {
                watch_path: dirs::download_dir()
                    .expect("Failed to get default download dir")
                    .to_string_lossy()
                    .to_string(),
                recursive: false,
            },
            formats: HashMap::new(),
            profile: Profile {
                username: String::new(),
                password: String::new(),
            },
        }
    }
}
