use crate::schema::todos::dsl::todos as all_todos;
use crate::{db::connect_db, schema::todos};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug, Serialize)]
#[diesel(table_name = crate::schema::todos)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Todo {
    id: i32,
    text: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "todos"]
pub struct NewTodo {
    text: String,
}

impl Todo {
    pub fn get_all() -> Vec<Todo> {
        let conn = &mut connect_db();

        all_todos
            .load::<Todo>(conn)
            .expect("Failed to get all users!")
    }

    pub fn add_todo(todo: NewTodo) {
        let conn = &mut connect_db();

        diesel::insert_into(todos::table)
            .values(&todo)
            .execute(conn)
            .expect("Failed adding todo!");
    }
}
