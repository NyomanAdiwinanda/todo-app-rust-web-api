use crate::{
    entities::schema::todos,
    models::todo_model::{NewTodo, Todo, TodosResponse},
};
use diesel::prelude::*;
use diesel::result::Error;

pub struct TodoRepository;

impl TodoRepository {
    pub fn find_all(c: &mut SqliteConnection, limit: i64) -> QueryResult<Vec<TodosResponse>> {
        todos::table
            .select((todos::id, todos::task, todos::created_at, todos::updated_at))
            .order(todos::id.desc())
            .limit(limit)
            .load::<TodosResponse>(c)
    }

    pub fn find_one(c: &mut SqliteConnection, id: i32) -> QueryResult<Todo> {
        todos::table.find(id).get_result::<Todo>(c)
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
        let todo = todos::table.find(id).first::<Todo>(c);

        match todo {
            Ok(_) => diesel::delete(todos::table.find(id)).execute(c),
            Err(Error::NotFound) => Ok(0),
            Err(e) => Err(e),
        }
    }
}
