// use super::error::LLError;
use super::error::{LLResult, LLError};
use std::{
    fs,
    collections::HashMap
};

#[derive(Debug)]
pub struct Epw {
    pub id: u32,
    pub mna: String, // Manufacturer?
    pub mpn: String, // Manufacturer part number
    pub pna: String, // Download initiator? Eg Mouser
    pub w: String,
    pub pc: u32,
    pub sym: u32,
    pub fmt: u32,
    pub ck: String
}

impl Epw {

    pub fn from_file<S: Into<String>>(p: S) -> LLResult<Self> {

        let f_data = fs::read(p.into())?;
        let f_str = String::from_utf8_lossy(&f_data).to_string();

        Self::from_string(f_str)

    }

    pub fn from_string<S: Into<String>>(d: S) -> LLResult<Self> {

        let data = d.into();
        let mut lines = data.lines();

        let mut map = HashMap::<&str, &str>::new();

        // let id = lines.next()?.parse::<u32>()?;

        let id = match lines.next() {
            Some(v) => v.parse::<u32>()?,
            None => {
                return Err(LLError::new("No data in input file"))
            }
        };

        for line in lines {

            let line_parts: Vec<&str> = line.split("=").collect();

            if line_parts.len() == 2 {
                map.insert(line_parts[0], line_parts[1]);
            }

        }

        Ok(Self {
            id: id,
            mna: String::from(*map.get("mna").unwrap_or(&"")),
            mpn: String::from(*map.get("mpn").unwrap_or(&"")),
            pna: String::from(*map.get("pna").unwrap_or(&"")),
            w: String::from(*map.get("w").unwrap_or(&"")),
            pc: map.get("pc").unwrap_or(&"0").parse::<u32>().unwrap_or(0),
            sym: map.get("sym").unwrap_or(&"0").parse::<u32>().unwrap_or(0),
            fmt: map.get("fmt").unwrap_or(&"0").parse::<u32>().unwrap_or(0),
            ck: String::from(*map.get("ck").unwrap_or(&""))
        })

    }

    pub fn from_id<I: Into<u32>>(id: I) -> Self {
        Self {
            id: id.into(),
            mna: String::new(),
            mpn: String::new(),
            pna: String::new(),
            w: String::new(),
            pc: 0,
            sym: 0,
            fmt: 0,
            ck: String::new()
        }
    }

}
