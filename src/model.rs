use std::{io::Error, path::{Path, PathBuf}, process::Command, rc::Rc};
use crate::savefile::{self, Combination, SaveFile, Word};

pub struct Model {
    model_path: PathBuf,
    pub save_file: SaveFile,
}

pub trait ModelTrait {
    fn new(model_path: PathBuf, save_file: SaveFile) -> Self;
    fn query(&self, input1: &str, input2: &str) -> Result<Word, Error>;
    fn query_savefile(&self, combination: &Combination) -> Option<&Word>;
    fn combine_into_result(&self, combination: &Combination) -> Word {
        let first_word = &combination.0;
        let second_word = &combination.1;
        match self.query_savefile(combination) {
            Some(recipe) => recipe.clone(),
            None => {
                let result = self.query(&first_word, &second_word).unwrap();
                result
            },
        }
    }
}

impl ModelTrait for Model {
    fn new(model_path: PathBuf, save_file: SaveFile) -> Self {
        Model {
            model_path,
            save_file,
        }
    }
    fn query(&self, input1: &str, input2: &str) -> Result<Word, Error> {
        let output = Command::new(&self.model_path)
            .arg(input1)
            .arg(input2)
            .output()?;
        
    let stdout = String::from_utf8_lossy(&output.stdout);
    let first_line = stdout.lines().next().unwrap_or("");
    Ok(Rc::from(first_line))
    }
    fn query_savefile(&self, combination: &Combination) -> Option<&Word> {
        self.save_file.recipe_result(combination)
    }
}

pub struct FakeTestModel {
    pub save_file: SaveFile,
}

impl ModelTrait for FakeTestModel {
    fn new(_model_path: PathBuf, save_file: SaveFile) -> Self {
        FakeTestModel {
            save_file,
        }
    }
    fn query(&self, input1: &str, input2: &str) -> Result<Word, Error> {
        let output = format!("{}{}", input1, input2);
        let mut chars: Vec<char> = output.chars().collect();
        chars.sort();
        let output: String = chars.into_iter().collect();
        Ok(Rc::from(output))
    }
    fn query_savefile(&self, combination: &Combination) -> Option<&Word> {
        self.save_file.recipe_result(combination)
    }
}