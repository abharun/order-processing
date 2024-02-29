use crate::*;

pub fn run_line(line: &str, tl: &mut TodoList) {
    if let Ok((_, q)) = parser::query(line) {
        match run_query(q, tl) {
            Ok(r) => {
                println!("{}", r);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}

fn run_query(q: Query, tl: &mut TodoList) -> Result<QueryResult, QueryError> {
    match q {
        Query::Add(desc, tags) => {
            let result = tl.push(desc, tags);
            return Ok(QueryResult::Added(result));
        }
        Query::Done(idx) => {
            let result = tl.done_with_index(idx);

            match result {
                Some(_result) => Ok(QueryResult::Done),
                None => Err(QueryError(q.to_string())),
            }
        }
        Query::Search(params) => {
            let result = tl.search(params);
            return Ok(QueryResult::Found(result));
        }
    }
}
