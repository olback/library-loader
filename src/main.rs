/**
 * Library Loader
 */

mod error;
mod profile;
mod epw;
mod cse;
mod cse_result;
mod config;
mod consts;

use config::Config;
use epw::Epw;
use cse::CSE;
use error::LLResult;

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

    if conf.generate_config {

        return match Config::generate() {
            Ok(v) => {
                println!("Generated {}", consts::LL_CONFIG);
                Ok(v)
            },
            Err(e) => Err(e)
        }

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

    // Create CSE
    let component_search_engine = CSE::new(&conf.profile);

    // Attempt to download lib
    let res = match component_search_engine.get(e) {
        Ok(v) => v,
        Err(e) => return Err(e)
    };

    match res.save(&conf) {
        Ok(p) => println!("File downloaded to '{}'", p),
        Err(e) => return Err(e)
    };

    Ok(())

}
