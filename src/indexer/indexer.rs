use std::collections::HashMap;

use super::tokenizer::FileToken;

pub fn index(files: Vec<FileToken>, path: &String) -> IIndex {
    let mut index = IIndex {
        path: path.clone(),
        files: Vec::new(),
    };
    for file in files {
        let mut tokens = HashMap::new();
        for token in file.tokens {
            let frq = match tokens.get(&token) {
                Some(s) => s,
                None => &0,
            };
            tokens.insert(token, frq + 1);
        }
        index.files.push(IFileIndex {
            path: file.path,
            tokens,
        });
    }
    index
}

/// pub tokens: HashMap<String, u32>
#[derive(Debug, Clone)]
pub struct IFileIndex {
    pub path: String,
    pub tokens: HashMap<String, u32>
}


/// pub path: String,
/// pub files: Vec<IFileIndex>,
///
/// IFileIndex
/// tokens: HashMap<String, u32>
#[derive(Debug, Clone)]
pub struct IIndex {
    pub path: String,
    pub files: Vec<IFileIndex>,
}

