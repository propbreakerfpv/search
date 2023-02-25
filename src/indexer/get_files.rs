use std::fs;

/// path: String,
/// content: String,
#[derive(Debug)]
pub struct FileText {
    pub path: String,
    pub content: String,
}

pub fn read_files<T: ToString>(path: &T) -> Result<Vec<FileText>, String> {
    let folder = match fs::read_dir(path.to_string()) {
        Ok(v) => v,
        Err(e) => return Err(e.to_string()),
    };
    let mut out = Vec::new();
    for entry in folder {
        let entry = match entry {
            Ok(v) => v,
            Err(e) => return Err(e.to_string()),
        };
        let path = entry.path();

    let p = match path.to_str() {
            Some(s) => s,
            None => return Err("error converting path to str".to_string()),
        };
        if path.is_dir() {
            // maybe do some recurtion to get all sub dirs as well?
            let ret = match read_files(&p) {
                Ok(v) => v,
                Err(e) => return Err(e.to_string()),
            };
            for f in ret {
                out.push(f);
            }
        } else {

            let content = match read_file(p) {
                Ok(v) => v,
                Err(e) => return Err(e.to_string()),
            };

            let ft = FileText {
                path: p.to_string(),
                content
            };

            out.push(ft);
        }
    }
    Ok(out)
}


/// does nothing exept read the file. this could maybe be inpruved in future by adding file type
/// detection and say parsing only the text nodes from a XML file.
fn read_file<T>(path: T) -> Result<String, std::io::Error> 
where T: ToString 
{
    fs::read_to_string(path.to_string())
}
