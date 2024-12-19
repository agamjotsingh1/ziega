use crate::parse::Command;
use colored::Colorize;

#[derive(Debug)]
pub struct Err {
    command: Command,
    pointer: usize,
    msg: String,
}

impl Err {
    pub fn new(command: Command, pointer: usize, msg: &str) -> Self {
        Err { command, pointer, msg: msg.to_string() }
    }

    pub fn throw(&self) {
        println!("{} {}", "error:".red().bold(), self.msg);
        let loc: usize = (self.command.args.as_ref().unwrap().as_slice())[..self.pointer]
            .iter()
            .map(|arg| arg.len() + 1)
            .sum();
        let kind_len = self.command.raw.split_whitespace().next().unwrap().len() + 1;
        let arg_len = self.command.args.as_ref().unwrap()[self.pointer].len();

        println!("{} {}", ">".blue(), self.command.raw);

        println!("{}{}", " ".repeat("> ".len() + kind_len + loc), "^".repeat(arg_len).red());
    }
}
