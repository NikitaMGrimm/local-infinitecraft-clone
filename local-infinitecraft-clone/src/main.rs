use std::{env, process::Command, path::{Path, PathBuf}};
use crate::model::{FakeTestModel, Model, ModelTrait};

pub mod model;

fn main() {
    let llm_bin = Path::new("/home/nikita/.local/bin/mistral");
    
    let ingredient1 = "apple";
    let ingredient2 = "tree";

    let model = FakeTestModel::new(llm_bin);

    let output = model.query(ingredient1, ingredient2).unwrap();
    println!("{}", output);
}