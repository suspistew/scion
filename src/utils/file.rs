use std::{
    env,
    fs::File,
    io::Read,
    path,
    path::{Path, PathBuf},
    time::SystemTime,
};

#[derive(Debug)]
pub struct FileReaderError {
    _msg: String,
}

/// This will read a file and return it as a byte vec.
pub fn read_file(path: &Path) -> Result<Vec<u8>, FileReaderError> {
    let mut file = open_file(path)?;
    let mut buffer = Vec::new();
    let read_result = file.read_to_end(&mut buffer);
    match read_result {
        Ok(_) => Ok(buffer),
        Err(e) => Err(FileReaderError { _msg: e.to_string() }),
    }
}

pub fn read_file_modification_time(path: &Path) -> Result<SystemTime, FileReaderError> {
    let file = open_file(path)?;
    match file.metadata() {
        Ok(metadata) => Ok(metadata.modified().unwrap()),
        Err(e) => Err(FileReaderError { _msg: e.to_string() }),
    }
}

pub fn open_file(path: &Path) -> Result<File, FileReaderError> {
    match File::open(path) {
        Ok(file) => Ok(file),
        Err(e) => Err(FileReaderError { _msg: e.to_string() }),
    }
}

/// This will give you the path to the executable (when in build mode) or to the root of the current project.
pub fn app_base_path() -> PathBuilder {
    if let Some(manifest_dir) = env::var_os("CARGO_MANIFEST_DIR") {
        return PathBuilder { path_buff: path::PathBuf::from(manifest_dir) };
    }

    match env::current_exe() {
        Ok(path) => PathBuilder { path_buff: path },
        Err(e) => {
            log::error!("Error while creating the app base_path {:?}, will use default.", e);
            PathBuilder { path_buff: Default::default() }
        }
    }
}

/// This will give you the path to the executable (when in build mode) or to the root of the current project.
/// joined with the giver path
pub fn app_base_path_join(path: &str) -> String {
    if let Some(manifest_dir) = env::var_os("CARGO_MANIFEST_DIR") {
        return PathBuilder { path_buff: path::PathBuf::from(manifest_dir) }.join(path).get();
    }

    match env::current_exe() {
        Ok(p) => PathBuilder { path_buff: p }.join(path).get(),
        Err(e) => {
            log::error!("Error while creating the app base_path {:?}, will use default.", e);
            PathBuilder { path_buff: Default::default() }.join(path).get()
        }
    }
}

/// Utils to help to build path and get them as String
pub struct PathBuilder {
    path_buff: PathBuf,
}

impl PathBuilder {
    pub fn join(mut self, path: &str) -> PathBuilder {
        self.path_buff = self.path_buff.join(path);
        self
    }

    pub fn get(self) -> String {
        self.path_buff
            .as_path()
            .to_str()
            .expect("Unable to extract the path from the path builder")
            .to_string()
    }
}
