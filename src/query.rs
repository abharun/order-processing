use std::fmt;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Query {
    Add (Description, Vec<Tag>),
    Done (Index),
    Search (SearchParams),
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::Add(desc, tags) => write!(f, "add \"{}\" {:?}", desc, tags),
            Self::Done(idx) => write!(f, "done {}", idx),
            Self::Search(params) => {
                let mut result: String = String::from("search ");
                if params.words.len() > 0 {
                    params.words.iter().for_each(|word| { result += word.value(); });
                }
                if params.tags.len() > 0 {
                    params.tags.iter().for_each(|tag| { result += tag.value(); });
                }
                write!(f, "search {}", result)
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchParams {
    pub words : Vec<SearchWord>,
    pub tags  : Vec<Tag>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchWord(String);
impl SearchWord {
    pub fn new(s: &str) -> SearchWord {
        SearchWord(s.to_owned())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueryResult {
    Added (TodoItem),
    Done,
    Found (Vec<todo_list::TodoItem>),
}

impl fmt::Display for QueryResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            QueryResult::Added(ti) => write!(f, "{}", ti.index),
            QueryResult::Done => write!(f, "done"),
            QueryResult::Found(rs) => {
                let mut buff : Vec<String> = vec![];
                buff.push(format!("{} item(s) found", rs.len()));
                for i in rs {
                    buff.push(format!("{}", i.to_string()));
                }
                write!(f, "{}", buff.join("\n"))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryError(pub String);

impl fmt::Display for QueryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An error occurred while processing the query: {}.", self.0)
    }
}
