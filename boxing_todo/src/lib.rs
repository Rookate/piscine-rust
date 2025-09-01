pub mod err;
pub use crate::err::{ParseErr, ReadErr};
pub use json::{parse, stringify};
pub use std::error::Error;
use std::{fs::File, io::Read};

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        eprintln!("[get_todo] Opening file: {}", path);

        let mut file = File::open(path).map_err(|e| {
            eprintln!("[get_todo] ERROR opening file: {e}");
            Box::new(ReadErr {
                child_err: Box::new(e),
            }) as Box<dyn Error>
        })?;

        eprintln!("[get_todo] File opened successfully.");

        let mut s = String::new();
        file.read_to_string(&mut s).map_err(|e| {
            eprintln!("[get_todo] ERROR reading file: {e}");
            Box::new(ReadErr {
                child_err: Box::new(e),
            }) as Box<dyn Error>
        })?;

        eprintln!(
            "[get_todo] File read successfully, length={} chars",
            s.len()
        );

        if s.trim().is_empty() {
            eprintln!("[get_todo] ERROR: file is empty after trim");
            return Err(Box::new(ParseErr::Empty));
        }

        eprintln!("[get_todo] Raw content:\n{}", s);

        let parsed_json = parse(&s).map_err(|e| {
            eprintln!("[get_todo] ERROR parsing JSON: {:?}", e);
            Box::new(ParseErr::Malformed(Box::new(e))) as Box<dyn Error>
        })?;

        eprintln!("[get_todo] JSON parsed successfully: {:?}", parsed_json);

        let title = parsed_json["title"]
            .as_str()
            .ok_or_else(|| {
                eprintln!("[get_todo] ERROR: missing or invalid title field");
                Box::new(ParseErr::Empty) as Box<dyn Error>
            })?
            .to_string();

        eprintln!("[get_todo] Title parsed: {}", title);

        if parsed_json["tasks"].len() == 0 {
            eprintln!("[get_todo] ERROR: no tasks found");
            return Err(Box::new(ParseErr::Empty));
        }

        eprintln!(
            "[get_todo] Found {} tasks, start parsing...",
            parsed_json["tasks"].len()
        );

        let mut tasks = Vec::new();
        for i in 0..parsed_json["tasks"].len() {
            eprintln!("[get_todo] Parsing task {}", i);

            let id = parsed_json["tasks"][i]["id"].as_u32().ok_or_else(|| {
                eprintln!("[get_todo] ERROR: invalid id at task {}", i);
                Box::new(ParseErr::Empty) as Box<dyn Error>
            })?;

            let desc = parsed_json["tasks"][i]["description"]
                .as_str()
                .ok_or_else(|| {
                    eprintln!("[get_todo] ERROR: invalid description at task {}", i);
                    Box::new(ParseErr::Empty) as Box<dyn Error>
                })?;

            let level = parsed_json["tasks"][i]["level"].as_u32().ok_or_else(|| {
                eprintln!("[get_todo] ERROR: invalid level at task {}", i);
                Box::new(ParseErr::Empty) as Box<dyn Error>
            })?;

            eprintln!(
                "[get_todo] Parsed task {} → id={}, desc='{}', level={}",
                i, id, desc, level
            );

            tasks.push(Task {
                id,
                description: desc.to_string(),
                level,
            });
        }

        eprintln!("[get_todo] SUCCESS: parsed {} tasks", tasks.len());

        Ok(TodoList { title, tasks })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn it_works() {
        let files = [
            (
                "todo.json",
                r#"{
                "title" : "TODO LIST FOR PISCINE RUST",
                "tasks": [
                    { "id": 0, "description": "do this", "level": 0 },
                    { "id": 1, "description": "do that", "level": 5 }
                ]
            }"#,
            ),
            (
                "todo_empty.json",
                r#"{
                "title" : "TODO LIST FOR PISCINE RUST",
                "tasks": []
            }"#,
            ),
            (
                "malformed_object.json",
                r#"{
                "something": ,
            }"#,
            ),
        ];

        for (name, content) in files {
            File::create(name)
                .unwrap()
                .write(content.as_bytes())
                .unwrap();

            let todos = TodoList::get_todo(name);
            match todos {
                Ok(list) => println!("{:?}", list),
                Err(e) => {
                    println!("{}: {:?}", e.to_string(), e.source());
                }
            }
        }
    }
}
