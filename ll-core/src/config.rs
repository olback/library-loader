use super::{
    new_err,
    profile::Profile,
    consts::LL_CONFIG,
    error::LLResult,
    format::Format
};
use serde::{Serialize, Deserialize};
use std::{
    fs,
    path::PathBuf
};
use dirs;
use toml;

#[cfg(feature = "cli-opts")]
use clap::{
    self,
    load_yaml,
    crate_version
};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParseConfig {
    pub settings: Option<ParseSettings>,
    pub profile: Option<Profile>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParseSettings {
    pub output_path: Option<String>,
    pub watch_path: Option<String>, // If set, enable watch mode
    pub format: Option<String> // If set, extract relevant files and place them in output_path
}

#[derive(Debug, Clone)]
pub struct Settings {
    pub output_path: String,
    pub watch_path: Option<String>,
    pub format: Format
}

#[derive(Debug, Clone)]
pub struct Cli {
    pub input: String,
    pub generate_config: (bool, bool), // (generate config?, save to users config dir?)
    pub treat_input_as_id: bool
}

#[derive(Debug, Clone)]
pub struct Config {
    pub settings: Settings,
    pub profile: Profile,
    pub cli: Cli,
    pub config_file: Option<PathBuf>
}

impl Config {

    #[cfg(feature = "cli-opts")]
    pub fn load() -> Self {

        let conf: Self;

        let yml = load_yaml!("../../cli.yml");
        let matches = clap::App::from(yml).version(crate_version!()).get_matches();

        let internal = Self::try_from_fs(matches.value_of("config"));

        // This needs to be refactored
        if internal.is_ok() {
            let (int, int_path) = internal.unwrap();

            let settings = int.settings.map_or(Self::default().settings, |s| Settings {
                output_path: s
                    .output_path
                    .map_or(Self::default().settings.output_path, |v| String::from(v)),
                watch_path: s.watch_path.map(|v| String::from(v)),
                format: s
                    .format
                    .map_or(Self::default().settings.format, |v| Format::from(v)),
            });

            let profile = int.profile.unwrap_or(match matches.is_present("generate") {
                true => Profile::new("", ""),
                false => Profile::prompt(),
            });

            conf = Self {
                settings: Settings {
                    output_path: matches
                        .value_of("output")
                        .map_or(settings.output_path, |v| String::from(v)),
                    watch_path: matches
                        .value_of("watch")
                        .map_or(settings.watch_path, |v| Some(String::from(v))),
                    format: matches
                        .value_of("format")
                        .map_or(settings.format, |v| Format::from(v)),
                },
                cli: Cli {
                    input: matches
                        .value_of("INPUT")
                        .map_or(Self::default().cli.input, |v| String::from(v)),
                    generate_config: (
                        matches.is_present("generate"),
                        matches.is_present("home_dir"),
                    ),
                    treat_input_as_id: matches.is_present("id"),
                },
                profile: profile,
                config_file: Some(int_path),
            }
        } else {
            conf = Self {
                settings: Settings {
                    output_path: matches
                        .value_of("output")
                        .map_or(Self::default().settings.output_path, |v| String::from(v)),
                    watch_path: matches
                        .value_of("watch")
                        .map_or(Self::default().settings.watch_path, |v| {
                            Some(String::from(v))
                        }),
                    format: matches
                        .value_of("format")
                        .map_or(Self::default().settings.format, |v| Format::from(v)),
                },
                cli: Cli {
                    input: matches
                        .value_of("INPUT")
                        .map_or(Self::default().cli.input, |v| String::from(v)),
                    generate_config: (
                        matches.is_present("generate"),
                        matches.is_present("home_dir"),
                    ),
                    treat_input_as_id: matches.is_present("id"),
                },
                profile: match matches.is_present("generate") {
                    true => Profile::new("", ""),
                    false => Profile::prompt(),
                },
                config_file: None,
            };
        }

        #[cfg(debug_assertions)]
        {
            println!("-- Debug info from {file}#{line} --", file = std::file!(), line = std::line!());
            println!("{:#?}", conf);
        }

        conf
    }

    #[cfg(not(feature = "cli-opts"))]
    pub fn load() -> Self {

        let default = Self::default();

        match Self::try_from_fs(None) {
            Ok((v, p)) => {
                let settings = match v.settings {
                    Some(fs) => {
                        Settings {
                            output_path: fs.output_path.unwrap_or(default.settings.output_path),
                            watch_path: fs.watch_path.or(default.settings.watch_path),
                            format: match fs.format {
                                Some(f) => Format::from(f),
                                None => default.settings.format
                            }
                        }
                    },
                    None => default.settings
                };
                Self {
                    settings: settings,
                    profile: v.profile.unwrap_or(Profile::new("", "")),
                    cli: Cli {
                        input: String::new(),
                        generate_config: (false, false),
                        treat_input_as_id: false
                    },
                    config_file: Some(p)
                }
            },
            Err(e) => {
                new_err!(e);
                default
            }
        }

    }

    fn try_from_fs(path_input: Option<&str>) -> LLResult<(ParseConfig, PathBuf)> {

        let path = match path_input {
            Some(p) => p,
            None => LL_CONFIG
        };

        let mut conf: Option<(ParseConfig, PathBuf)> = None;

        let pb = PathBuf::from(path);
        if pb.exists() {
            let data = fs::read(path)?;
            conf = Some((toml::from_slice(&data)?, pb));
        }

        // Don't bother checking global config is local is already set.
        if conf.is_none() {

            // Check home dir for LL Config.
            let home_path = match dirs::config_dir() {
                Some(hp) => Some(hp.join(LL_CONFIG)),
                None => None
            };

            match home_path {
                Some(v) => {
                    let data = fs::read(&v)?;
                    conf = Some((toml::from_slice(&data)?, v));
                },
                None => {}
            };

        }

        match conf {
            Some(c) => Ok(c),
            None => Err(new_err!(format!("{} not found", LL_CONFIG)))
        }

    }

    pub fn generate(global: &bool, input: &String) -> LLResult<String> {

        let path = match global {
            true => {
                match dirs::config_dir() {
                    Some(p) => p.join(LL_CONFIG),
                    None => {
                        eprintln!("Global config path could not be determined, saving to working directory");
                        PathBuf::from(LL_CONFIG)
                    }
                }
            },
            false => {
                PathBuf::from(match input.trim().is_empty() {
                    true => LL_CONFIG,
                    false => input
                })
            }
        };

        if path.clone().exists() {

            return Err(new_err!(format!("{} already exists", path.to_str().unwrap())));

        }

        match fs::write(&path, include_str!("../../LibraryLoader.example.toml")) {
            Ok(_) => {
                let path_as_string = String::from(path.to_str().unwrap());
                Ok(path_as_string)
            },
            Err(e) => Err(new_err!(e))
        }

    }

}

impl Default for Config {

    fn default() -> Self {

        let profile = Profile::new("", "");

        Self {
            settings: Settings {
                output_path: String::from("download"),
                watch_path: None,
                format: Format::from("zip")
            },
            profile: profile,
            cli: Cli {
                input: String::new(),
                generate_config: (false, false),
                treat_input_as_id: false
            },
            config_file: None
        }

    }

}
