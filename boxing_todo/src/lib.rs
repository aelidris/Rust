mod err;

use std::{error::Error, fs, io};

use json::{parse, JsonValue};

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
        let content = fs::read_to_string(path).map_err(|e| {
            Box::new(err::ReadErr {
                child_err: Box::new(e),
            }) as Box<dyn Error>
        })?;

        let parsed = parse(&content).map_err(|e| {
            Box::new(err::ParseErr::Malformed(Box::new(e))) as Box<dyn Error>
        })?;

        if !parsed.has_key("title") || !parsed.has_key("tasks") {
            return Err(Box::new(err::ParseErr::Malformed(Box::new(io::Error::new(
                io::ErrorKind::InvalidData,
                "Missing 'title' or 'tasks' field",
            )))));
        }

        let title = parsed["title"].as_str().ok_or_else(|| {
            Box::new(err::ParseErr::Malformed(Box::new(io::Error::new(
                io::ErrorKind::InvalidData,
                "Title is not a string",
            )))) as Box<dyn Error>
        })?;

        let tasks_array = &parsed["tasks"];
        if tasks_array.is_empty() {
            return Err(Box::new(err::ParseErr::Empty));
        }

        let mut tasks = Vec::new();
        for task in tasks_array.members() {
            let id = task["id"].as_u32().ok_or_else(|| {
                Box::new(err::ParseErr::Malformed(Box::new(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Task id is missing or not a number",
                )))) as Box<dyn Error>
            })?;

            let description = task["description"].as_str().ok_or_else(|| {
                Box::new(err::ParseErr::Malformed(Box::new(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Task description is missing or not a string",
                )))) as Box<dyn Error>
            })?;

            let level = task["level"].as_u32().ok_or_else(|| {
                Box::new(err::ParseErr::Malformed(Box::new(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Task level is missing or not a number",
                )))) as Box<dyn Error>
            })?;

            tasks.push(Task {
                id,
                description: description.to_string(),
                level,
            });
        }

        Ok(TodoList {
            title: title.to_string(),
            tasks,
        })
    }
}