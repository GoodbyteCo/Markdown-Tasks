use std::fs;
use markdown_tasks::*;


#[test]
fn pasre(){
    let unparsed_file = fs::read_to_string("tests/test.md").expect("cannot read file");

    let main = TaskParser::parse(Rule::main, &unparsed_file)
        .expect("unsuccessful parse") // unwrap the parse result
        .next().unwrap();
    dbg!(&main);
    assert!(true);
}