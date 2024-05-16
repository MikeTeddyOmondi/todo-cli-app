mod actions;
mod db;
mod utils;

use rusqlite::Connection;
use std::env;

use crate::actions::{add_action, complete_action, delete_action, show_action, Action};

fn main() {
    println!("Todo CLI Application [Built w/ Rust]");

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
       return utils::print_help();
    }

    let action_arg = args[1].trim().to_lowercase().to_string();

    let action: Action = match action_arg {
        add if add == "add".to_string() => Action::Add,
        show if show == "show".to_string() => Action::Show,
        help if help == "help".to_string() => Action::Help,
        delete if delete == "delete".to_string() => Action::Delete,
        complete if complete == "complete".to_string() => Action::Complete,
        _ => Action::Help,
    };

    let conn: Connection;
    let result = db::init();

    match result {
        Ok(connection) => {
            // println!("Db initialised...");
            conn = connection;
            // println!("conn: {:?}", conn);

            match action {
                Action::Add => add_action(&conn),
                Action::Show => show_action(&conn),
                Action::Complete => complete_action(&conn),
                Action::Delete => delete_action(&conn),
                Action::Help => utils::print_help(),
            }
        }
        Err(err) => return println!("Error initialising database connection: {}", err),
    }
}
