use axum::{response::{Html, IntoResponse}, routing::{get, post, delete, put}, Router, Form, extract::{Path, Query}};
use askama::Template;
use serde::{Deserialize, Serialize};
use crate::db::{read_todos_from_file, write_todos_to_file, initialize_todos};

pub mod templates;
pub mod db;

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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Todo {
    id: usize,
    text: String,
    completed: bool,
}


#[derive(Deserialize, Serialize, Debug)]
struct NewTodo {
    text: String,
}

async fn handler() -> Html<String> {
    let template = templates::Index{
        show_active: true,
        show_complete: true,
    };
    Html(template.render().unwrap())
}

async fn active() -> impl IntoResponse {
    println!("called active endpoint");
    let template = templates::Index{
        show_active: true,
        show_complete: false,
    };
    Html(template.render().unwrap())
}

async fn completed() -> impl IntoResponse {
    println!("called completed endpoint");
    let template = templates::Index{
        show_active: false,
        show_complete: true,
    };
    Html(template.render().unwrap())
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
    let template = templates::Todos::new(todos, show_active, show_complete);
    Html(template.render().unwrap())
}


async fn todos() -> Html<String> {
    let todos = read_todos_from_file("todos.json").unwrap();
    let template = templates::Todos::new(todos, true, true);
    Html(template.render().unwrap())
}

async fn add_todo(Form(to_add): Form<NewTodo>) -> impl IntoResponse {
    let mut todo = read_todos_from_file("todos.json").unwrap();
    let new_id = todo.len() + 1;
    let new_text = to_add.text.clone().trim().to_string();
    if new_text != "" {
        let new_todo = Todo {
            id: new_id,
            text: new_text.trim().to_string(),
            completed: false,
        };
        todo.push(new_todo);
        println!("New todos {:#?}", todo);
        write_todos_to_file("todos.json", &todo).unwrap();
    }
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
    let new_text = to_edit.text.trim().to_string();
    if new_text == "" {
        todo.retain(|todo| todo.id != id);
    } else {
        for t in todo.iter_mut() {
            if t.id == id {
                t.text = new_text;
                break;
            }
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
    let template = templates::EditTodo {
        todo: todo.clone(),
    };
    Html(template.render().unwrap())

}