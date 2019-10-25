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

use config::Config;
use epw::Epw;
use cse::CSE;
use error::{LLResult, LLError};
use watcher::Watcher;

fn main() {

    match real_main() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1)
        }
    }

}

fn real_main() -> LLResult<()> {

    // Load config
    let conf = Config::load();

    // Create CSE
    let component_search_engine = CSE::new(&conf);

    if conf.generate_config {

        return match Config::generate(&conf.input) {
            Ok(v) => {
                println!("Generated {}", v);
                Ok(())
            },
            Err(e) => Err(e)
        }

    } else if conf.settings.watch_path.is_some() && conf.input.is_empty() {

        let p = conf.settings.watch_path.clone().unwrap();
        let mut w = Watcher::new(p, component_search_engine)?;
        let tx = w.get_tx();

        // React on key input
        std::thread::spawn(move || {
            #[allow(unused_must_use)]
            {
                std::io::stdin().read_line(&mut String::new());
                tx.send(Err(notify::Error::generic("stop")));
            }
        });

        println!("Watching {}", &conf.settings.watch_path.unwrap());
        println!("Saving to: {}", &conf.settings.output_path);
        println!("Saving in format: {:?}", &conf.settings.format);
        println!("Press <Enter> to exit");

        w.start()?;

    } else if conf.input.is_empty() {

        let args: Vec<String> = std::env::args().collect();
        return Err(LLError::new(format!("No input specified, run `{} --help` for more help", args[0])))

    } else {

        if conf.settings.watch_path.is_some() {
            println!("Ignoring watch command since input was supplied");
        }

        let e = match conf.treat_input_as_id {
            true => {
                Epw::from_id(conf.input.parse::<u32>()?)
            },
            false => {
                match Epw::from_file(&conf.input) {
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
