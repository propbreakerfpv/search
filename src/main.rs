use std::env;

mod indexer;
mod caching;
mod search;

fn main() {
    
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        help();
        return;
    }

    match args[1].as_str() {
        "--index" => {
            if args.len() > 3 {
                index(&args[2]);
            } else {
                help();
            }
        }
        "--list" => {
            list();
        }
        _ => search(args),
    };


}

fn help() {
    println!("Usage: search <path> <search term>");
    println!("Sub Commands");
    println!("  --index <path>              - index the given path and save it so it can be searched later");
    println!("  --update <optional path>    - reindex a path or if no path is provided reindex all indexed paths");
}

fn list() {
    caching::read_write::get_all_cached_paths().iter().for_each(|x| {
        println!("{}", x);
    });
}


fn search(args: Vec<String>) {
    if args.len() < 2 {
        help();
        return;
    }
    let path = &args[1];

    search::search(path, args[1..].join(" "))
    // println!("{:?}", caching::read_write::read_cach(path.to_string()));
}


fn index(path: &String) {

    println!("indexing {}", path);

    let files = crate::indexer::get_files::read_files(&path).unwrap();
    let indexes = crate::indexer::tokenizer::tokenize(files);
    let indexed = crate::indexer::indexer::index(indexes, &path);

    let sorted = crate::indexer::sorter::sort_inter_index(indexed, &path);
    println!("{:?}", sorted);

    caching::read_write::write_cach(path.to_string(), sorted).unwrap_or_else(|e| println!("{}",e.to_string()));
}
