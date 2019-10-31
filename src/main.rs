/**
 * Library Loader
 */

mod error;
mod profile;
mod epw;
mod cse;
mod cse_result;
mod config;
mod format;
mod consts;
mod watcher;
mod check_updates;

use config::Config;
use epw::Epw;
use cse::CSE;
use error::{LLResult, LLError};
use watcher::Watcher;
use std::io::Read;

fn main() {

    // Check for updates
    let update_handle = std::thread::spawn(move || {
        match check_updates::check() {
            Ok(available) => {
                match available {
                    Some(update) => {
                        println!("New update available! {} -> {}", update.local, update.remote);
                        println!("{}", consts::DOWNLOAD_URL);
                    },
                    None => {}
                }
            },
            Err(e) => {
                eprintln!("{}#{}: Error checking for updates: {}", std::file!(), std::line!(), e);
            }
        }
    });

    match real_main() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1)
        }
    }

    update_handle.join().unwrap();

}

fn real_main() -> LLResult<()> {

    // Load config
    let conf = Config::load();

    // Create CSE
    let component_search_engine = CSE::new(&conf);

    if conf.cli.generate_config.0 {

        return match Config::generate(&conf.cli.generate_config.1, &conf.cli.input) {
            Ok(v) => {
                println!("Generated {}", v);
                Ok(())
            },
            Err(e) => Err(e)
        }

    } else if conf.settings.watch_path.is_some() && conf.cli.input.is_empty() {

        let p = conf.settings.watch_path.clone().unwrap();
        let mut w = Watcher::new(p, component_search_engine)?;
        let tx = w.get_tx();

        // React on key input
        let input_handle = std::thread::spawn(move || {

            std::io::stdin().read(&mut [0u8]).unwrap();
            tx.send(Err(notify::Error::generic("stop"))).unwrap();

        });

        let watch_path_buf = std::path::PathBuf::from(&conf.settings.watch_path.unwrap());
        let save_to_buf = std::path::PathBuf::from(&conf.settings.output_path);

        println!("Watching {}", watch_path_buf.as_path().canonicalize()?.to_string_lossy());
        println!("Saving to: {}", save_to_buf.as_path().canonicalize()?.to_string_lossy());
        println!("Saving in format: {:?}", &conf.settings.format.ecad);
        println!("Press <Enter> to exit");

        w.start()?;
        input_handle.join().unwrap();

    } else if conf.cli.input.is_empty() {

        let args: Vec<String> = std::env::args().collect();
        return Err(LLError::new(format!("No input specified, run `{} --help` for more help", args[0])))

    } else {

        if conf.settings.watch_path.is_some() {
            println!("Ignoring watch command since input was supplied");
        }

        let e = match conf.cli.treat_input_as_id {
            true => {
                Epw::from_id(conf.cli.input.parse::<u32>()?)
            },
            false => {
                match Epw::from_file(&conf.cli.input) {
                    Ok(v) => v,
                    Err(e) => return Err(e)
                }
            }
        };

        // Attempt to download lib
        let res = match component_search_engine.get(e) {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        match res.save() {
            Ok(p) => println!("File(s) downloaded to '{}'", p),
            Err(e) => return Err(e)
        };

    }

    Ok(())

}
