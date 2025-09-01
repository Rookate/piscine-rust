mod err;
use err::{ParseErr, ReadErr};

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
        let mut file = File::open(path).map_err(|e| {
            Box::new(ReadErr {
                child_err: Box::new(e),
            }) as Box<dyn Error>
        })?;
        let mut s = String::new();
        file.read_to_string(&mut s).map_err(|e| {
            Box::new(ReadErr {
                child_err: Box::new(e),
            }) as Box<dyn Error>
        })?;
        if s.trim().is_empty() {
            return Err(Box::new(ParseErr::Empty));
        }
        let parsed_json = parse(&s).map_err(|e| Box::new(ParseErr::Malformed(Box::new(e))))?;
        let title = parsed_json["title"]
            .as_str()
            .ok_or_else(|| Box::new(ParseErr::Empty))?
            .to_string();

        let mut tasks: Vec<Task> = Vec::new();
        if parsed_json["tasks"].len() == 0 {
            return Err(Box::new(ParseErr::Empty));
        }

        for i in 0..parsed_json["tasks"].len() {
            let task = Task {
                id: parsed_json["tasks"][i]["id"]
                    .as_u32()
                    .ok_or(Box::new(ParseErr::Empty))?,
                description: parsed_json["tasks"][i]["description"]
                    .as_str()
                    .ok_or_else(|| Box::new(ParseErr::Empty))?
                    .to_string(),
                level: parsed_json["tasks"][i]["level"]
                    .as_u32()
                    .ok_or_else(|| Box::new(ParseErr::Empty))?,
            };
            tasks.push(task)
        }
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
