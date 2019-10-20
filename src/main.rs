/**
 * Library Loader
 */

// use clap::{App, load_yaml};

mod error;
mod profile;
mod epw;
mod cse;

use profile::Profile;
use epw::Epw;
use cse::CSE;

fn main() {

    // let yaml = load_yaml!("../cli.yml");
    // let input = App::from_yaml(&yaml).get_matches();
    // let part_id = input.args.get("part_id").unwrap();
    // println!("{:#?}", &part_id);

    let p = Profile::new("username", "password");
    let e = Epw::from_file("MAX3232ECD-pcb-part-libraries.epw").unwrap();
    let s = CSE::new(p);
    let res = s.get(e);

    match res {
        Ok(_) => println!("Download complete!"),
        Err(e) => eprintln!("{:#?}", e)
    }

}
