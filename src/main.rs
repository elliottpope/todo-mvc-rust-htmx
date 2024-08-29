use axum::{response::{Html, IntoResponse}, routing::{get, post, delete, put}, Router, Form, extract::{Path, Query}};
use askama::Template;
use serde::{Deserialize, Serialize};
use std::fs::{File, copy};
use std::io::{BufReader, BufWriter};

#[tokio::main]
async fn main() {

    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        // .route("/#", get(handler))
        .route("/active", get(active))
        .route("/completed", get(completed))
        .route("/todos", get(get_all_todos)
            .delete(delete_completed_todos)
            .put(update_todos))
        .route("/todo", post(add_todo))
        .route("/todo/:id/edit", get(edit_todo))
        .route("/todo/:id", 
            delete(delete_todo)
            .put(update_todo)
            .post(update_todo_text));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    
    let _ = initialize_todos("init.json", "todos.json").await;
    
    axum::serve(listener, app).await.unwrap();
}

async fn initialize_todos(source: &str, destination: &str) {
    copy(source, destination).unwrap();
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index {
    show_active: bool,
    show_complete: bool,
}

#[derive(Template)]
#[template(path = "todo-list.html")]
struct Todos {
    todo: Vec<Todo>,
    done: Vec<Todo>,
    show_active: bool,
    show_complete: bool,
    total: usize,
}

impl Todos {
    fn new(todos: Vec<Todo>, show_active: bool, show_complete: bool) -> Self {
        let total = todos.len();
        let (done, todo): (Vec<Todo>, Vec<Todo>) = todos.into_iter()
            .partition(|todo| todo.completed);
        Todos {
            show_active: show_active,
            show_complete: show_complete,
            done: done,
            todo: todo,
            total: total,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Todo {
    id: usize,
    text: String,
    completed: bool,
}

#[derive(Template)]
#[template(path = "edit-todo.html")]
struct EditTodo {
    todo: Todo
}

#[derive(Deserialize, Serialize, Debug)]
struct NewTodo {
    text: String,
}

async fn handler() -> Html<String> {
    let template = Index{
        show_active: true,
        show_complete: true,
    };
    Html(template.render().unwrap())
}

async fn active() -> impl IntoResponse {
    println!("called active endpoint");
    let template = Index{
        show_active: true,
        show_complete: false,
    };
    Html(template.render().unwrap())
}

async fn completed() -> impl IntoResponse {
    println!("called completed endpoint");
    let template = Index{
        show_active: false,
        show_complete: true,
    };
    Html(template.render().unwrap())
}

fn read_todos_from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<Todo>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let todos: Vec<Todo> = serde_json::from_reader(reader)?;
    Ok(todos)
}

fn write_todos_to_file<P: AsRef<std::path::Path>>(path: P, todos: &Vec<Todo>) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, todos)?;
    Ok(())
}

#[derive(Deserialize, Serialize, Debug)]
struct TodosQuery {
    active: Option<bool>,
    complete: Option<bool>,
}

async fn get_all_todos(Query(show): Query<TodosQuery>) -> impl IntoResponse {
    let show_active = show.active.unwrap_or(true);
    let show_complete = show.complete.unwrap_or(true);

    let todos = read_todos_from_file("todos.json").unwrap();
    let template = Todos::new(todos, show_active, show_complete);
    Html(template.render().unwrap())
}


async fn todos() -> Html<String> {
    let todos = read_todos_from_file("todos.json").unwrap();
    let template = Todos::new(todos, true, true);
    Html(template.render().unwrap())
}

async fn add_todo(Form(to_add): Form<NewTodo>) -> impl IntoResponse {
    let mut todo = read_todos_from_file("todos.json").unwrap();
    let new_id = todo.len() + 1;
    let new_text = to_add.text.clone();
    let new_todo = Todo {
        id: new_id,
        text: new_text.trim().to_string(),
        completed: false,
    };
    todo.push(new_todo);
    println!("New todos {:#?}", todo);
    write_todos_to_file("todos.json", &todo).unwrap();
    todos().await
}

async fn delete_todo(Path(id): Path<usize>) -> impl IntoResponse {
    let mut todo = read_todos_from_file("todos.json").unwrap();
    todo.retain(|todo| todo.id != id);
    println!("New todos {:#?}", todo);
    write_todos_to_file("todos.json", &todo).unwrap();
    todos().await
}

async fn delete_completed_todos() -> impl IntoResponse {
    let mut todo = read_todos_from_file("todos.json").unwrap();
    todo.retain(|todo| !todo.completed);
    println!("New todos {:#?}", todo);
    write_todos_to_file("todos.json", &todo).unwrap();
    todos().await
}

#[derive(Deserialize)]
struct TodoStatusUpdateRequest {
    status: Option<String>,
    text: Option<String>,
}

async fn update_todo(Path(id): Path<usize>, Query(status): Query<TodoStatusUpdateRequest>) -> impl IntoResponse {
    let mut todo = read_todos_from_file("todos.json").unwrap();
    let completed = "complete".to_string();
    let todo_string = "todo".to_string();
    for t in todo.iter_mut() {
        if t.id == id {
            if status.status == Some(completed) {
                t.completed = true
            } else if status.status == Some(todo_string) {
                t.completed = false
            }

            if status.text.is_some() {
                t.text = status.text.unwrap();
            }
            break;
        }
    }
    println!("New todos {:#?}", todo);
    write_todos_to_file("todos.json", &todo).unwrap();
    todos().await
}

async fn update_todo_text(Path(id): Path<usize>, Form(to_edit): Form<NewTodo>) -> impl IntoResponse {
    let mut todo = read_todos_from_file("todos.json").unwrap();
    for t in todo.iter_mut() {
        if t.id == id {
            t.text = to_edit.text;
            break;
        }
    }
    println!("New todos {:#?}", todo);
    write_todos_to_file("todos.json", &todo).unwrap();
    todos().await
}

async fn update_todos(Query(status): Query<TodoStatusUpdateRequest>) -> impl IntoResponse {
    let mut todo = read_todos_from_file("todos.json").unwrap();
    let completed = Some("complete".to_string());
    for t in todo.iter_mut() {
        if status.status == completed {
            t.completed = true
        }
    }
    println!("New todos {:#?}", todo);
    write_todos_to_file("todos.json", &todo).unwrap();
    todos().await
}

async fn edit_todo(Path(id): Path<usize>) -> impl IntoResponse {
    let todos = read_todos_from_file("todos.json").unwrap();
    let todo = match todos.iter().find(|&todo| todo.id == id) {
        Some(todo) => todo,
        None => return Html("".to_string())
    };
    let template = EditTodo {
        todo: todo.clone(),
    };
    Html(template.render().unwrap())

}