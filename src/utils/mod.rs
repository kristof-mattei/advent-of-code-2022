use std::{fs::File, io::Read, path::PathBuf};

pub fn read_file(path_to_file: PathBuf) -> Result<Vec<String>, std::io::Error> {
    let mut file = File::open(path_to_file)?; // nasty path resolve
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents.lines().map(Into::into).collect())
}
