use std::{path::{Path, PathBuf}, process::Command, io::Error};

pub struct Model {
    model_path: PathBuf
}

pub trait ModelTrait {
    fn new<P: AsRef<Path>>(model_path: P) -> Self;
    fn query(&self, input1: &str, input2: &str) -> Result<String, Error>;
}

impl ModelTrait for Model {
    fn new<P: AsRef<Path>>(model_path: P) -> Self {
        Model {
            model_path: model_path.as_ref().to_path_buf()
        }
    }
    fn query(&self, input1: &str, input2: &str) -> Result<String, Error> {
        let output = Command::new(&self.model_path)
            .arg(input1)
            .arg(input2)
            .output()?;
        
    let stdout = String::from_utf8_lossy(&output.stdout);
    let first_line = stdout.lines().next().unwrap_or("");
    Ok(first_line.to_string())
    }
}
