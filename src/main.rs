use std::{env, process::Command, path::{Path, PathBuf}};
use crate::model::{FakeTestModel, Model, ModelTrait};

pub mod model;

fn main() {
    let args: Vec<String> = env::args().collect();
    let llm_bin = Path::new("/home/nikita/.local/bin/mistral");
    
    let model = FakeTestModel::new(llm_bin);

    let output = model.query(&args[1], &args[2]).unwrap();
    println!("{}", output);
}