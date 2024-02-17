use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;
use std::rc::Rc;

use bincode::serialize_into;
use serde::{Deserialize, Serialize};

pub type Word = Rc<str>; // maybe Box<> if I dont need clone

#[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Combination(pub Word, pub Word);

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveFile {
    unlocked: HashSet<Word>,
    recipes: HashMap<Combination, Word>,
}

impl SaveFile {
    pub fn new() -> Self {
        SaveFile {
            unlocked: HashSet::new(),
            recipes: HashMap::new(),
        }
    }
    pub fn unlock_word(&mut self, word: Word) -> bool {
        self.unlocked.insert(word)
    }
    pub fn is_unlocked(&self, word: &Word) -> bool {
        self.unlocked.contains(word)
    }
    pub fn insert_recipe(&mut self, combination: Combination, result_word: Word) -> &Word {
        self.recipes.entry(combination).or_insert(result_word)
    }
    pub fn recipe_result(&self, combination: &Combination) -> Option<&Word> {
        self.recipes.get(combination)
    }
    pub fn save_to_file(&self, save_file_path: PathBuf) {
        let encoded = bincode::serialize(&self).unwrap();
        let file = File::create(save_file_path).unwrap();
        let writer = BufWriter::new(file);
        serialize_into(writer, &encoded).unwrap();
    }
    pub fn load_from_file(save_file_path: PathBuf) -> Self {
        let file = File::open(save_file_path).unwrap();
        let reader = BufReader::new(file);
        let decoded: SaveFile = bincode::deserialize_from(reader).unwrap();
        decoded
    }
}