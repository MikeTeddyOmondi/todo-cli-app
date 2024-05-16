use rusqlite::{Connection, Error, Result};

pub mod models {
    #[derive(Debug, Clone)]
    pub struct Todo {
        pub id: String,
        pub title: String,
        pub completed: bool,
    }
}

pub fn add_todo(conn: &Connection, todo: models::Todo) -> Result<usize, Error> {
    // println!("Todo: {:?}", todo);
    let result = conn.execute(
        "INSERT INTO todos (id, title, completed) values (?1, ?2, ?3)",
        [todo.id, todo.title, (todo.completed as i32).to_string()],
    )?;
    Ok(result)
}

pub fn complete_todo(conn: &Connection, id: String) -> Result<usize, Error> {
    let result = conn.execute("UPDATE todos set completed = 1 WHERE (id) = (?1);", [id])?;
    Ok(result)
}

pub fn delete_todo(conn: &Connection, id: String) -> Result<usize, Error> {
    let result = conn.execute("DELETE FROM todos WHERE (id) = (?1);", [id])?;
    Ok(result)
}

pub fn show_all(conn: &Connection) -> Result<Vec<models::Todo>, Error> {
    let mut stmt = conn.prepare("SELECT * from todos;")?;

    let todos = stmt.query_map((), |row| {
        Ok(models::Todo {
            id: row.get(0)?,
            title: row.get(1)?,
            completed: row.get(2)?,
        })
    })?;

    let mut todos_array = Vec::new();

    for todo in todos {
        match todo {
            Ok(todo) => todos_array.push(todo),
            Err(err) => println!("Error occurred: {:?}", err),
        }
    }

    Ok(todos_array)
}

pub fn show_one(conn: &Connection, id: String) -> Result<Vec<models::Todo>, Error> {
    let mut stmt = conn.prepare("SELECT * from todos WHERE (id) = :id;")?;

    let todos = stmt.query_map(&[(":id", &id)], |row| {
        Ok(models::Todo {
            id: row.get(0)?,
            title: row.get(1)?,
            completed: row.get(2)?,
        })
    })?;

    let mut todos_array = Vec::new();

    for todo in todos {
        match todo {
            Ok(todo) => todos_array.push(todo),
            Err(err) => println!("Error occurred: {:?}", err),
        }
    }

    Ok(todos_array)
}

pub fn init() -> Result<Connection, Error> {
    let conn = Connection::open("./todos.db")?;

    conn.execute(
        "create table if not exists todos (
             id text primary key,
             title text not null,
             completed boolean not null 
        )",
        (),
    )?;

    Ok(conn)
}
