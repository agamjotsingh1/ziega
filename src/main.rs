mod err;
mod parse;

fn main() {
    let test: err::Err = err::Err::new(parse::Command::new("help please?"), 0, "lol");
    test.throw();
}
