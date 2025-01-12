use crate::{
    entities::schema::todos,
    models::todo_model::{NewTodo, Todo},
};
use diesel::prelude::*;

pub struct TodoRepository;

impl TodoRepository {
    pub fn find_one(c: &mut SqliteConnection, id: i32) -> QueryResult<Todo> {
        todos::table.find(id).get_result::<Todo>(c)
    }

    pub fn find_all(c: &mut SqliteConnection, limit: i64) -> QueryResult<Vec<Todo>> {
        todos::table
            .order(todos::id.desc())
            .limit(limit)
            .load::<Todo>(c)
    }

    pub fn create(c: &mut SqliteConnection, new_todo: NewTodo) -> QueryResult<Todo> {
        diesel::insert_into(todos::table)
            .values(new_todo)
            .execute(c)
            .expect("DB error when inserting todo");

        todos::table.order(todos::id.desc()).first::<Todo>(c)
    }

    pub fn update(c: &mut SqliteConnection, id: i32, todo: Todo) -> QueryResult<Todo> {
        diesel::update(todos::table.find(id))
            .set((
                todos::task.eq(todo.task.to_owned()),
                todos::description.eq(todo.description.to_owned()),
            ))
            .execute(c)
            .expect("DB error when updating todo");

        Self::find_one(c, id)
    }

    pub fn delete(c: &mut SqliteConnection, id: i32) -> QueryResult<usize> {
        todos::table
            .find(id)
            .get_result::<Todo>(c)
            .expect("DB error when deleting todo");

        diesel::delete(todos::table.find(id)).execute(c)
    }
}
