use super::get_files::FileText;

/// path: String
/// tokens: Vec<String>
#[derive(Debug)]
pub struct FileToken {
    pub path: String,
    pub tokens: Vec<String>,
}

pub fn tokenize(files: Vec<FileText>) -> Vec<FileToken>{
    let mut ftokens = Vec::new();
    for text in files {
        let mut parcer = Parcer {
            content: text.content,
            pos: 0,
            path: text.path.clone(),
        };
        ftokens.push(parcer.parce(text.path))
    }
    ftokens
}

struct Parcer {
    content: String,
    pos: usize,
    path: String,
}

impl Parcer {
    pub fn parce(&mut self, path: String) -> FileToken {
        println!("parcing {}", path);
        let mut tokens = Vec::new();
        loop {
            self.consume_whitespace();
            if self.end() {
                break;
            }
            tokens.push(self.parse_next());
        }
        FileToken {
            path,
            tokens
        }
    }

    fn parse_next(&mut self) -> String {
        match self.next_char() {
            'a'..='z' | 'A'..='Z' => self.parse_word(),
            '0'..='9' => self.parse_number(),
            _ => self.parse_unknown(),
        }
    }
    
    fn parse_unknown(&mut self) -> String {
        self.consume_while(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | ' ' | '\n' => false,
            _ => true,
        })
    }


    fn parse_number(&mut self) -> String {
        self.consume_while(|c| match c {
            '0'..='9' | '.' | ',' => true,
            _ => false,
        })
    }

    fn parse_word(&mut self) -> String {
        self.consume_while(|c| match c {
            'a'..='z' | 'A'..='Z' => true,
            _ => false,
        })
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(|c| c.is_whitespace());
    }
    fn consume_while<F>(&mut self, test: F) -> String 
        where F: Fn(char) -> bool
    {
        let mut str = String::new();
        if self.end() {
            return str;
        }
        while ! self.end() && test(self.next_char()) {
            str.push(self.consume_char());
            if self.end() {
                break;
            }
        }
        str
    }

    fn consume_char(&mut self) -> char {
        self.pos += 1;
        match self.content.chars().nth(self.pos - 1) {
            Some(s) => s,
            None => panic!("next char not found consume"),
        }
    }

    fn next_char(&self) -> char {
        match self.content.chars().nth(self.pos) {
            Some(s) => s,
            None => panic!("next char not found next"),
        }
    }

    fn end(&self) -> bool {
        self.pos >= self.content.chars().count()
    }
}
