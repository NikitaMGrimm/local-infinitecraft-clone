use std::{collections::{HashMap, HashSet}, env, fs::File, io::Write, path::Path, rc::Rc};
use crate::{model::{FakeTestModel, Model, ModelTrait}, savefile::{Combination, SaveFile}};
use serde_json;

pub mod model;
pub mod savefile;
extern crate serde;

fn main() {
    let args: Vec<String> = env::args().collect();
    let llm_bin = Path::new("/home/nikita/.local/bin/mistral");
    let save_file_path = Path::new("savefile");
    let save_file: SaveFile;
        
    if !save_file_path.exists() {
        save_file = SaveFile::new();
    } else {
        save_file = SaveFile::load_from_file(save_file_path.to_path_buf());
    }
        
    let mut model = FakeTestModel::new(llm_bin.to_path_buf(), save_file);

    let combination = Combination(Rc::from(args[1].clone()), Rc::from(args[2].clone()));

    let result = model.combine_into_result(&combination);
    model.save_file.unlock_word(result.clone());
    model.save_file.insert_recipe(combination, result);

    dbg!(
        "Save file: {:#?}",
        &model.save_file,
    );

    let _ = model.save_file.save_to_file(save_file_path.to_path_buf());
}