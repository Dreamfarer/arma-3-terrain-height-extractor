use arma_rs::{arma, Extension};
use std::{fs::OpenOptions, io::Write};

#[arma]
fn init() -> Extension {
    Extension::build()
        .command("csv_write", csv_write)
        .command("csv_init", csv_init)
        .finish()
}

pub fn csv_init(file_name: String) -> Result<String, String> {
    let mut file = match OpenOptions::new().write(true).create(true).truncate(true).open(&file_name) {
        Ok(f) => f,
        Err(_) => return Err(String::from("File not found!"))
    };
    if let Err(_) = writeln!(file, "x,y,height") {
        return Err(String::from("Failed to write to the file!"));
    }
    Ok(format!("Successfully initialized {}!", &file_name))
}

pub fn csv_write(file_name: String, data: String) -> Result<String, String> {
    let mut file = match OpenOptions::new().append(true).open(&file_name) {
        Ok(f) => f,
        Err(_) => return Err(String::from("File not found!"))
    };
    if let Err(_) = writeln!(file, "{}", &data) {
        return Err(String::from("Failed to write to the file!"));
    }
    Ok(format!("Successfully written to {}!", &file_name))
}