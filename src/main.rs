#[macro_use]
extern crate prettytable;

use prettytable::Table;
use rusqlite::{Connection, Error, Result};
use std::{
    env,
    io::{self, Write},
};
use uuid::Uuid;

#[derive(Debug, Clone)]
struct Todo {
    id: String,
    title: String,
    completed: bool,
}

#[derive(Debug)]
enum Actions {
    ADD,
    SHOW,
    HELP,
    DELETE,
    COMPLETE,
}

fn init_db() -> Result<Connection, Error> {
    let conn = Connection::open("todos.db")?;

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

fn print_help() {
    println!("___________________________________");
    println!("Usage: bin [ACTION]");
    println!();
    println!("ACTIONS:");
    println!("  help        Show help menu");
    println!("  add         Add todos");
    println!("  show        Show todos");
    println!("  complete    Complete a todo");
    println!("  delete      Delete a todo");
}

fn add_todo(conn: Connection, todo: Todo) -> Result<usize, Error> {
    // println!("Todo: {:?}", todo);
    let result = conn.execute(
        "INSERT INTO todos (id, title, completed) values (?1, ?2, ?3)",
        [todo.id, todo.title, (todo.completed as i32).to_string()],
    )?;
    Ok(result)
}

fn complete_todo(conn: &Connection, id: String) -> Result<usize, Error> {
    let result = conn.execute("UPDATE todos set completed = 1 WHERE (id) = (?1);", [id])?;
    Ok(result)
}

fn delete_todo(conn: &Connection, id: String) -> Result<usize, Error> {
    let result = conn.execute("DELETE FROM todos WHERE (id) = (?1);", [id])?;
    Ok(result)
}

fn show_all(conn: Connection) -> Result<Vec<Todo>, Error> {
    let mut stmt = conn.prepare("SELECT * from todos;")?;

    let todos = stmt.query_map((), |row| {
        Ok(Todo {
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

fn show_one(conn: &Connection, id: String) -> Result<Vec<Todo>, Error> {
    let mut stmt = conn.prepare("SELECT * from todos WHERE (id) = :id;")?;

    let todos = stmt.query_map(&[(":id", &id)], |row| {
        Ok(Todo {
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

fn user_input(text: String) -> String {
    println!("___________________________________");
    let mut input_buffer = String::new();
    print!("Enter {}: ", text);
    let _ = io::stdout().flush(); // glues the previous print! statements to the stdin
    let bytes_read = io::stdin().read_line(&mut input_buffer).unwrap();
    println!("***********************************");
    // println!("Todo: {}", line.trim());
    println!("No. of bytes read: {}", bytes_read);
    println!("___________________________________");
    input_buffer
}

fn main() {
    println!("Todo CLI Application [Built w/ Rust]");

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        print_help();
        return;
    }

    let action_arg = args[1].trim().to_lowercase().to_owned();

    let add_value: String = "add".to_owned();
    let show_value: String = "show".to_owned();
    let help_value: String = "help".to_owned();
    let delete_value: String = "delete".to_owned();
    let complete_value: String = "complete".to_owned();

    let action: Actions = match action_arg {
        add if add == add_value => Actions::ADD,
        show if show == show_value => Actions::SHOW,
        help if help == help_value => Actions::HELP,
        delete if delete == delete_value => Actions::DELETE,
        complete if complete == complete_value => Actions::COMPLETE,
        _ => Actions::HELP,
    };

    let conn: Connection;
    let result = init_db();

    match result {
        Ok(connection) => {
            println!("Db initialised...");
            conn = connection;
            println!("conn: {:?}", conn);

            match action {
                Actions::ADD => {
                    println!("Add Action: ");
                    let text = "your todo".to_owned();
                    let input = user_input(text);

                    let todo = Todo {
                        id: Uuid::new_v4().to_string(),
                        title: input.trim().to_owned(),
                        completed: false,
                    };

                    let db_result = add_todo(conn, todo.clone());

                    match db_result {
                        Ok(usize) => {
                            println!("Rows inserted: {:?}", usize);
                            println!("Todo added!");

                            // Create table
                            let mut todos_table = Table::new();

                            // Add a row per time
                            todos_table.add_row(row![b->"ID", b->"Title", b->"Completed"]);
                            todos_table.add_row(row![todo.id, todo.title, todo.completed]);

                            // Print the table to stdout
                            todos_table.printstd();
                        }
                        Err(err) => {
                            println!("An error occured while inserting database record: {}", err)
                        }
                    }
                }
                Actions::SHOW => {
                    println!("Show Action:");
                    println!("___________________________________");
                    let db_results = show_all(conn);

                    match db_results {
                        Ok(v) => {
                            println!("Rows returned: {:?}", v.len());
                            // println!("Todos: {:?}", v);

                            if v.len() == 0 {
                                println!("No todos found!");
                                return;
                            };

                            // Create the table
                            let mut todos_table = Table::new();

                            // Add a row per time
                            todos_table.add_row(row![b->"ID", b->"Title", b->"Completed"]);

                            for Todo {
                                id,
                                title,
                                completed,
                            } in v.iter()
                            {
                                todos_table.add_row(row![id, title, completed]);
                            }

                            // Print the table to stdout
                            todos_table.printstd();
                        }
                        Err(err) => {
                            println!("An error occured while querying database records: {}", err)
                        }
                    }
                }
                Actions::COMPLETE => {
                    println!("Complete Action");
                    let text = "ID".to_owned();
                    let input = user_input(text);

                    let id = input.trim().to_owned();

                    let db_result_complete = complete_todo(&conn, id.clone());

                    match db_result_complete {
                        Ok(usize) => {
                            println!("Rows updated: {:?}", usize);
                            println!("Todo completed!");
                            println!("___________________________________");
                        }
                        Err(err) => {
                            println!("An error occured while updating database record: {}", err)
                        }
                    }

                    let db_result_fetch = show_one(&conn, id.clone());

                    match db_result_fetch {
                        Ok(v) => {
                            println!("Rows returned: {:?}", v.len());
                            // println!("Todos: {:?}", v);

                            if v.len() == 0 {
                                println!("No todo found!");
                                return;
                            };

                            // Create the table
                            let mut todos_table = Table::new();

                            // Add a row per time
                            todos_table.add_row(row![b->"ID", b->"Title", b->"Completed"]);

                            for Todo {
                                id,
                                title,
                                completed,
                            } in v.iter()
                            {
                                todos_table.add_row(row![id, title, completed]);
                            }

                            // Print the table to stdout
                            todos_table.printstd();
                        }
                        Err(err) => {
                            println!("An error occured while querying database records: {}", err)
                        }
                    }
                }
                Actions::DELETE => {
                    println!("Delete Action");
                    let text = "ID".to_owned();
                    let input = user_input(text);

                    let id = input.trim().to_owned();

                    let db_result_fetch = show_one(&conn, id.clone());

                    match db_result_fetch {
                        Ok(v) => {
                            println!("Rows returned: {:?}", v.len());
                            // println!("Todos: {:?}", v);

                            if v.len() == 0 {
                                println!("No todo found!");
                                return;
                            };

                            // Create the table
                            let mut todos_table = Table::new();

                            // Add a row per time
                            todos_table.add_row(row![b->"ID", b->"Title", b->"Completed"]);

                            for Todo {
                                id,
                                title,
                                completed,
                            } in v.iter()
                            {
                                todos_table.add_row(row![id, title, completed]);
                            }

                            // Print the table to stdout
                            todos_table.printstd();
                        }
                        Err(err) => {
                            println!("An error occured while querying database records: {}", err)
                        }
                    }

                    let db_result_delete = delete_todo(&conn, id.clone());

                    match db_result_delete {
                        Ok(usize) => {
                            println!("Rows deleted: {:?}", usize);
                            println!("Todo deleted!");
                        }
                        Err(err) => {
                            println!("An error occured while deleting database record: {}", err)
                        }
                    }
                }
                Actions::HELP => print_help(),
            }
        }
        Err(err) => panic!("Error initialising database connection: {}", err),
    }
}
