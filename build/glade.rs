use regex;
use std::fs;

pub fn fix_resource_paths() {

    const GLADE_IN_PATH: &str = "assets/library-loader.glade";
    const GLADE_OUT_PATH: &str = "out/library-loader.glade";

    let glade_xml_data = fs::read_to_string(GLADE_IN_PATH).unwrap();
    let re = regex::Regex::new(r"(?P<r>resource:/)(?P<p>[a-z])").unwrap();
    let after = re.replace_all(&glade_xml_data, "$r//$p");

    fs::write(GLADE_OUT_PATH, after.to_owned().as_bytes()).unwrap();

}