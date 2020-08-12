extern crate pest;
#[macro_use]
extern crate pest_derive;

pub use pest::Parser;

#[derive(Parser)]
#[grammar = "task.pest"]
pub struct TaskParser;
