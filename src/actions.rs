use prettytable::{row, Table};
use rusqlite::Connection;
use uuid::Uuid;

use crate::db::models::*;
use crate::db::*;
use crate::utils::user_input;

#[derive(Debug)]
pub enum Action {
    Add,
    Show,
    Help,
    Delete,
    Complete,
}

pub fn add_action(conn: &Connection) {
    println!("Add: ");
    let text = "your todo".to_owned();
    let input = user_input(text);

    let todo = Todo {
        id: Uuid::new_v4().to_string(),
        title: input.trim().to_owned(),
        completed: false,
    };

    let db_result = add_todo(conn, todo.clone());

    match db_result {
        Ok(_usize) => {
            // println!("Rows inserted: {:?}", usize);
            // println!("Todo added!");

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

pub fn show_action(conn: &Connection) {
    println!("Show: ");
    // println!("___________________________________");
    let db_results = show_all(conn);

    match db_results {
        Ok(v) => {
            // println!("Rows returned: {:?}", v.len());
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

pub fn complete_action(conn: &Connection) {
    println!("Complete: ");
    let text = "ID".to_owned();
    let input = user_input(text);

    let id = input.trim().to_owned();

    let db_result_complete = complete_todo(&conn, id.clone());

    match db_result_complete {
        Ok(_usize) => {
            // println!("Rows updated: {:?}", usize);
            // println!("Todo completed!");
            println!("___________________________________");
        }
        Err(err) => {
            println!("An error occured while updating database record: {}", err)
        }
    }

    let db_result_fetch = show_one(&conn, id.clone());

    match db_result_fetch {
        Ok(v) => {
            //println!("Rows returned: {:?}", v.len());
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

pub fn delete_action(conn: &Connection) {
    println!("Delete: ");
    let text = "ID".to_owned();
    let input = user_input(text);

    let id = input.trim().to_owned();

    let db_result_fetch = show_one(&conn, id.clone());

    match db_result_fetch {
        Ok(v) => {
            // println!("Rows returned: {:?}", v.len());
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
        Ok(_usize) => {
            // println!("Rows deleted: {:?}", usize);
            // println!("Todo deleted!");
        }
        Err(err) => {
            println!("An error occured while deleting database record: {}", err)
        }
    }
}
