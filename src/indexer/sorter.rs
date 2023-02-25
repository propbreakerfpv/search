use std::collections::HashMap;


use serde_derive::{Serialize, Deserialize};

use super::indexer::{IIndex, IFileIndex};

pub fn sort_inter_index(iindex: IIndex, path: &String) -> Index {
    let mut index = Index {
        path: path.clone(),
        terms: get_terms(iindex.clone()),
        files: Vec::new(),
    };

    

    for i in iindex.files {
        index.files.push(sort_ifile_index(i));
    }

    index
}

fn get_terms(iindex: IIndex) -> Vec<Term> {
    let mut terms: HashMap<String, Term> = HashMap::new();
    for file in iindex.files {
        for t in file.tokens {
            let frq = match terms.get(&t.0) {
                Some(v) => v.frq + t.1,
                None => t.1
            };
            terms.insert(t.0.clone(), Term { value: t.0.clone(), frq});
        }
    }
    let mut terms = terms.iter().map(|x| x.1.clone()).collect::<Vec<Term>>();

    let mut i = 0;
    while i < terms.len() {
        let mut j = 0;
        while j < terms.len() - 1 - i {
            if terms[j].frq > terms[j + 1].frq {
                let tmp = terms[j].clone();
                terms[j] = terms[j + 1].clone();
                terms[j + 1] = tmp;
            }
            j += 1;
        }
        i += 1;
    }
    terms
}

fn sort_ifile_index(mut ifile: IFileIndex) -> FileIndex {
    let mut ifile_vec: Vec<Term> = ifile.tokens.drain().map(|c| Term { value: c.0, frq: c.1 }).collect();
    let mut i = 0;
    while i < ifile_vec.len() {
        let mut j = 0;
        while j < ifile_vec.len() - 1 - i {
            if ifile_vec[j].frq > ifile_vec[j + 1].frq {
                let tmp = ifile_vec[j].clone();
                ifile_vec[j] = ifile_vec[j + 1].clone();
                ifile_vec[j + 1] = tmp;
            }
            j += 1;
        }
        i += 1;
    }
    FileIndex {
        path: ifile.path,
        tokens: ifile_vec,
    }
}

/// pub path: String,
/// pub terms: Vec<Term>,
/// pub files: Vec<FileIndex>,
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Index {
    pub path: String,
    pub terms: Vec<Term>,
    pub files: Vec<FileIndex>,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileIndex {
    pub path: String,
    pub tokens: Vec<Term>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Term {
    pub value: String,
    pub frq: u32
}



impl Term {
    pub fn from_key_value(key: String, value: &str) -> Result<Term, String> {
        let value: u32 = match value.parse() {
            Ok(v) => v,
            Err(e) => return Err(e.to_string()),
        };
        Ok(Term {
            value: key,
            frq: value,
        })
    }
}

