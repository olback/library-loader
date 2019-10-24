// use super::error::LLError;
use super::error::{LLResult, LLError};
use std::{
    fs,
    collections::HashMap,
    ffi::OsStr,
    path::PathBuf,
    io::Read
};
use zip;

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
    pub ck: String,
    pub source: String
}

impl Epw {

    pub fn from_file<S: Into<PathBuf>>(path: S) -> LLResult<Self> {

        let p = path.into();
        let f_data = fs::read(&p)?;

        match Some(OsStr::new("zip")) == p.as_path().extension() {
            true => Self::from_zip(f_data),
            false => {
                let f_str = String::from_utf8_lossy(&f_data).to_string();
                Self::from_string(f_str)
            }
        }

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
            ck: String::from(*map.get("ck").unwrap_or(&"")),
            source: String::from(*map.get("source").unwrap_or(&""))
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
            ck: String::new(),
            source: String::new()
        }
    }

    fn from_zip(raw_data: Vec<u8>) -> LLResult<Self> {

        // println!("{:?}", &raw_data);

        // If the last byte is 0x0A, which it always seems to
        // be when downloading from Mouser, pop it and continue.
        // It's not supposed to be there and it have wasted half
        // a day trying to figure this out. Thanks Mouser.
        let data = match raw_data[raw_data.len() - 1] == 0x0A {
            true => {
                let mut data = raw_data.clone();
                data.pop();
                data
            },
            false => raw_data
        };

        let reader = std::io::Cursor::new(&data);
        let mut archive = zip::ZipArchive::new(reader)?;

        for i in 0..archive.len() {

            let mut file = archive.by_index(i)?;
            let path = PathBuf::from(file.name());

            if path.as_path().extension() == Some(OsStr::new("epw")) {

                let mut epw_content = String::new();
                file.read_to_string(&mut epw_content)?;

                return Self::from_string(epw_content)

            }

        }

        Err(LLError::new("No .epw file found in archive"))

    }

}
