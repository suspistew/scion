use std::{env, fs::File, io::Read, path, path::Path};
use std::path::PathBuf;

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
pub fn app_base_path() -> PathBuilder {
    if let Some(manifest_dir) = env::var_os("CARGO_MANIFEST_DIR") {
        return PathBuilder{ path_buff: path::PathBuf::from(manifest_dir)};
    }

    return match env::current_exe() {
        Ok(path) => PathBuilder{ path_buff: path},
        Err(e) => {
            log::error!("Error while creating the apbase_path {:?}, will use default.",e );
            PathBuilder{ path_buff: Default::default() }
        }
    };
}

pub struct PathBuilder{
    path_buff : PathBuf
}

impl PathBuilder {
    pub fn join(mut self, path: &str) -> PathBuilder{
        self.path_buff = self.path_buff.join(path);
        self
    }

    pub fn get(self) -> String {
        self.path_buff.as_path().to_str()
            .expect("Unable to extract the path from the path builder")
            .to_string()
    }
}