#[derive(Debug)]
pub enum Kind {
    HELP,
}

impl Kind {
    pub fn new(s: &str) -> Self {
        match s {
            "help" => Kind::HELP,
            _ => todo!()
        }
    }
}

#[derive(Debug)]
pub struct Command {
    pub raw: String,
    pub kind: Kind,
    pub args: Option<Vec<String>>,
}


impl Command {
    pub fn new(line: &str) -> Self {
        let mut args: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
        let kind: Kind = Kind::new(args.remove(0).as_str());
        
        Command { raw: line.to_string(), kind, args: Some(args) }
    }
}
