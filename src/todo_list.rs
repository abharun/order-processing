use std::fmt;

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Index(u64);

impl Index {
    pub fn new(i: u64) -> Index {
        Index(i)
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Description(String);

impl Description {
    pub fn new(s: &str) -> Description {
        Description(s.to_owned())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Description {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tag(String);

impl Tag {
    pub fn new(s: &str) -> Tag {
        Tag(s.to_owned())
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    pub fn from_strings(ss: Vec<&str>) -> Vec<Tag> {
        ss.clone().into_iter().map(|s| Tag::new(s)).collect()
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TodoItem {
    pub index: Index,
    pub description: Description,
    pub tags: Vec<Tag>,
    pub done: bool,
}

impl TodoItem {
    pub fn new(index: Index, description: Description, tags: Vec<Tag>, done: bool) -> TodoItem {
        TodoItem {
            index,
            description,
            tags,
            done,
        }
    }
}

impl fmt::Display for TodoItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        result += &format!("{} ", self.index.value());
        // result += &format!("\"{}\" ", self.description.value());
        // self.tags.iter().for_each(|tag| result += &format!("#{} ", tag.value()));
        write!(f, "{}", result)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TodoList {
    top_index: Index,
    items: Vec<TodoItem>,
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList {
            top_index: Index::new(0),
            items: vec![],
        }
    }

    pub fn push(&mut self, description: Description, tags: Vec<Tag>) -> TodoItem {
        let newitem: TodoItem = TodoItem {
            index: Index::new(self.top_index.value()),
            description: description,
            tags: tags,
            done: false,
        };

        self.items.push(newitem.clone());
        self.top_index = Index::new(self.top_index.value() + 1);
        return newitem;
    }

    pub fn done_with_index(&mut self, idx: Index) -> Option<Index> {
        let pos = self.items.iter().position(|item| item.index == idx);
        match pos {
            Some(pos) => {
                if self.items[pos].done == true {
                    return None;
                }
                self.items[pos].done = true;
                return Some(Index::new(pos as u64));
            }
            None => {
                return None;
            }
        }
    }

    pub fn search(&self, sp: SearchParams) -> Vec<TodoItem> {
        let mut result_items: Vec<TodoItem> = vec![];

        for item in self.items.iter() {
            if item.done == true {
                continue;
            }
            let descrips = item
                .description
                .value()
                .split_whitespace()
                .collect::<Vec<&str>>();
            if sp.words.iter().all(|word| {
                descrips
                    .iter()
                    .any(|itm_desc| check_string_involve(word.value(), itm_desc))
            }) && sp.tags.iter().all(|tag| {
                item.tags
                    .iter()
                    .any(|itm_tag| check_string_involve(tag.value(), itm_tag.value()))
            }) {
                result_items.push(item.clone());
            }
        }

        return result_items;
    }
}

fn check_string_involve(literal: &str, text: &str) -> bool {
    let literal_vec = literal.chars().collect::<Vec<char>>();
    let text_vec = text.chars().collect::<Vec<char>>();

    let mut literal_iter = literal_vec.iter();
    let mut text_iter = text_vec.iter();

    let mut literal_slt = literal_iter.next();

    while let Some(text_slt) = text_iter.next() {
        if literal_slt.unwrap() == text_slt {
            literal_slt = literal_iter.next();
        }

        if literal_slt == None {
            return true;
        }
    }

    false
}
