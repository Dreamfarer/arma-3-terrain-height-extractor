use arma_rs::{arma, Extension};
use std::{fs::OpenOptions, io::Write};

#[arma]
fn init() -> Extension {
    Extension::build()
        .command("csv_init", csv_init)
        .command("csv_write", csv_write)
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

pub fn csv_write(file_name: String, data: arma_rs::Value) -> Result<String, String> {
    let mut file = match OpenOptions::new().write(true).append(true).open(&file_name) {
        Ok(f) => f,
        Err(_) => return Err(String::from("File not found!")),
    };
    let data_vec = match data.as_vec() {
        Some(vec) => vec,
        None => return Err(String::from("Provided data is not an array!")),
    };
    let x_start = data_vec[0].as_f64().unwrap_or(0.0);
    let y = data_vec[1].as_f64().unwrap_or(0.0);
    let step_size = data_vec[2].as_f64().unwrap_or(0.0);
    let heights = &data_vec[3..];
    for (i, height) in heights.iter().enumerate() {
        let x = x_start + (i as f64) * step_size;
        let height_value = height.as_f64().unwrap_or(0.0);
        if let Err(_) = writeln!(file, "{},{},{}", x, y, height_value) {
            return Err(String::from("Failed to write data row to the file!"));
        }
    }
    Ok(format!("Successfully processed and written to {}!", &file_name))
}
