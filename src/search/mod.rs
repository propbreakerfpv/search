use crate::caching::read_write::read_cach;
use crate::indexer::sorter::{Index, Term, FileIndex};

pub fn search(path: &String, query: String) {
    let qtokens: Vec<String> = query.split(' ').collect::<Vec<&str>>().iter().map(|c| c.to_string()).collect();

    let to_search: Index = read_cach(path).unwrap();
    
    let mut out = Vec::new();

    for file in to_search.files {
        let filtered_len = file.tokens.iter().filter_map(|x| {
            if qtokens.contains(&x.value.clone()) {
                Some(x)
            }else {
                None
            }
        }).collect::<Vec<_>>().len();

        if filtered_len > 0 {
            out.push(file)
        }
    }


    for i in out {
        println!("{}", i.path);
    }

}
