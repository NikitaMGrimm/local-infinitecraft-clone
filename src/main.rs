use std::{collections::{HashMap, HashSet}, env, fs::File, io::Write, path::Path, rc::Rc};
use crate::{model::{FakeTestModel, Model, ModelTrait}, savefile::{Combination, SaveFile}};
use serde_json;

pub mod model;
pub mod savefile;

fn main() {
    let args: Vec<String> = env::args().collect();
    let llm_bin = Path::new("/home/nikita/.local/bin/mistral");
    
    let save_file = SaveFile::new();
    let model = FakeTestModel::new(llm_bin);

    let combination = Combination(Rc::from(args[1].clone()), Rc::from(args[2].clone()));

    println!("{output}", output = model.combine_into_result(&combination, &save_file_json));

    // Serialize save_file to JSON
    let save_file_json = serde_json::to_string(&save_file).unwrap();

    // Write save_file_json to savefile.json
    let mut file = File::create("savefile.json").unwrap();
    file.write_all(save_file_json.as_bytes()).unwrap();

    println!("{output}", output = model.combine_into_result(&combination, &save_file));

}