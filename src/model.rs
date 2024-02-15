use std::{io::Error, path::{Path, PathBuf}, process::Command, rc::Rc};
use crate::savefile::{Combination, SaveFile, Word};

pub struct Model {
    model_path: PathBuf,
    save_file: SaveFile,
}

pub trait ModelTrait {
    fn new<P: AsRef<Path>>(model_path: P) -> Self;
    fn query(&self, input1: &str, input2: &str) -> Result<Word, Error>;
    fn combine_into_result(&self, combination: &Combination, save_file: &SaveFile) -> Word {
        let first_word = &combination.0;
        let second_word = &combination.1;
        match save_file.recipe_result(combination) {
            Some(recipe) => recipe.clone(),
            None => {
                self.query(&first_word, &second_word).unwrap()
            },
        }
    }
}

impl ModelTrait for Model {
    fn new<P: AsRef<Path>>(model_path: P) -> Self {
        Model {
            model_path: model_path.as_ref().to_path_buf(),
            save_file: SaveFile::new(),
        }
    }
    fn query(&self, input1: &str, input2: &str) -> Result<Word, Error> {
        let output = Command::new(&self.model_path)
            .arg(input1)
            .arg(input2)
            .output()?;
        
    let stdout = String::from_utf8_lossy(&output.stdout);
    let first_line = stdout.lines().next().unwrap_or("");
    Ok(Rc::from(first_line.to_string()))
    }
}

pub struct FakeTestModel {
    pub save_file: SaveFile,
}

impl ModelTrait for FakeTestModel {
    fn new<P: AsRef<Path>>(_model_path: P) -> Self {
        FakeTestModel {
            save_file: SaveFile::new(),
        }
    }
    fn query(&self, input1: &str, input2: &str) -> Result<Word, Error> {
        let output = format!("{}{}", input1, input2);
        let mut chars: Vec<char> = output.chars().collect();
        chars.sort();
        let output: String = chars.into_iter().collect();
        Ok(Rc::from(output))
    }
}