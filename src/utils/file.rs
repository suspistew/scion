use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::{Error, Read};
use std::{path, io, env};

pub struct FileReaderError {
    msg: String
}

pub fn read_file(path: &Path) -> Result<Vec<u8>, FileReaderError>{
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => return Err(FileReaderError{ msg: e.to_string() })
    };
    let mut buffer = Vec::new();
    let read_result = file.read_to_end(&mut buffer);
    return match read_result {
        Ok(_) => Ok(buffer),
        Err(e) => Err(FileReaderError{ msg: e.to_string()})
    }
}

pub fn app_base_path() -> Result<path::PathBuf, io::Error> {
    if let Some(manifest_dir) = env::var_os("CARGO_MANIFEST_DIR") {
        return Ok(path::PathBuf::from(manifest_dir));
    }

    return match env::current_exe(){
        Ok(path) => Ok(path),
        Err(_) => Err(io::Error::new(io::ErrorKind::Other, "Failed to find an application root"))
    }
}
