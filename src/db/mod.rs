use std::fs::{File, copy};
use std::io::{BufReader, BufWriter};
use crate::Todo;

pub async fn initialize_todos(source: &str, destination: &str) {
    copy(source, destination).unwrap();
}

pub fn read_todos_from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<Todo>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let todos: Vec<Todo> = serde_json::from_reader(reader)?;
    Ok(todos)
}

pub fn write_todos_to_file<P: AsRef<std::path::Path>>(path: P, todos: &Vec<Todo>) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, todos)?;
    Ok(())
}