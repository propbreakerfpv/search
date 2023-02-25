

use std::{fs, collections::HashMap, path::PathBuf};


use serde_derive::{Serialize, Deserialize};

use crate::indexer::sorter::Index;


const CACH_FILE_PATH: &str = ".search";


/// # to many unwraps
pub fn write_cach(path: String, input: Index) -> Result<(), std::io::Error> {
    let read = fs::read_to_string(CACH_FILE_PATH)?;
    let mut current = match serde_json::from_str::<Cach>(&read[..]) {
        Ok(v) => v,
        Err(_) => {
            Cach {
                paths: HashMap::new(),
            }
        }
    };

    let path = fs::canonicalize(PathBuf::from(path)).unwrap().to_str().unwrap().to_string();

    current.paths.insert(path, input);


    let s = serde_json::to_string(&current).unwrap();

    fs::write(CACH_FILE_PATH, s)
}

pub fn read_cach(path: &String) -> Result<Index, std::io::Error>{
    let s = fs::read_to_string(CACH_FILE_PATH)?;
    let s = serde_json::from_str::<Cach>(&s[..]).unwrap();

    let path = fs::canonicalize(PathBuf::from(path)).unwrap();

    let t = s.paths.get(path.to_str().unwrap()).unwrap();

    Ok(t.clone())
}


pub fn get_all_cached_paths() -> Vec<String> {
    let s = fs::read_to_string(CACH_FILE_PATH).unwrap();
    let s = serde_json::from_str::<Cach>(&s[..]).unwrap();

    s.paths.keys().map(|x| x.clone()).collect::<Vec<String>>()
}


#[derive(Debug, Serialize, Deserialize, Clone)]
struct Cach {
    paths: HashMap<String, Index>
}



