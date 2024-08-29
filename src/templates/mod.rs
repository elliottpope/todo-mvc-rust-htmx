use askama::Template;
use crate::Todo;

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {
    pub show_active: bool,
    pub show_complete: bool,
}

#[derive(Template)]
#[template(path = "todo-list.html")]
pub struct Todos {
    pub todo: Vec<Todo>,
    pub done: Vec<Todo>,
    pub show_active: bool,
    pub show_complete: bool,
    pub total: usize,
}

impl Todos {
    pub fn new(todos: Vec<Todo>, show_active: bool, show_complete: bool) -> Self {
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

#[derive(Template)]
#[template(path = "edit-todo.html")]
pub struct EditTodo {
    pub todo: Todo
}
