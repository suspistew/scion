use std::{env, fs::File, io, io::Read, path, path::Path};

pub struct FileReaderError {
    _msg: String,
}

/// This will read a file and return it as a byte vec.
pub fn read_file(path: &Path) -> Result<Vec<u8>, FileReaderError> {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            return Err(FileReaderError {
                _msg: e.to_string(),
            })
        }
    };
    let mut buffer = Vec::new();
    let read_result = file.read_to_end(&mut buffer);
    return match read_result {
        Ok(_) => Ok(buffer),
        Err(e) => {
            Err(FileReaderError {
                _msg: e.to_string(),
            })
        }
    };
}

/// This will give you the path to the executable (when in build mode) or to the root of the current project.
pub fn app_base_path() -> Result<path::PathBuf, io::Error> {
    if let Some(manifest_dir) = env::var_os("CARGO_MANIFEST_DIR") {
        return Ok(path::PathBuf::from(manifest_dir));
    }

    return match env::current_exe() {
        Ok(path) => Ok(path),
        Err(_) => {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "Failed to find an application root",
            ))
        }
    };
}
