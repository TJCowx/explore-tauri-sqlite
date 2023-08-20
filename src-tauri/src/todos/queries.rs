use crate::todos::models::Todo;

use super::models::NewTodo;

#[tauri::command]
pub async fn add_todo(todo: NewTodo) {
    Todo::add_todo(todo)
}

#[tauri::command]
pub async fn get_todos() -> Result<Vec<Todo>, String> {
    let res = Todo::get_all();

    println!("Retrieved {} todos", res.len());

    Ok(res)
}
