#[macro_use]
extern crate mysql;

use mysql as my;
use std::env;

#[derive(Debug, PartialEq, Eq)]
struct Todo {
    id: i32,
    desc: Option<String>,
    action: i32,
}

fn print_todo(array: Vec<Todo>) {
    println!("");
    println!("{0: <10} | {1: <50} | {2: <10}", 
        "ID", "DESCRIPTION", "ACTION");
    
    for x in array {
        println!(
            "{0: <10} | {1: <50} | {2: <10}",
            x.id,
            x.desc.unwrap(),
            match x.action {
                0 => "Not Complete",
                1 => "Complete",
                _ => "Not Complete",
            }
        );
    }
    println!("")
}

fn main() {
    let pool = my::Pool::new("mysql://rust:rust@localhost:3306/rust_todo").unwrap();
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            let cmd = &args[1];
            match &cmd[..] {
                "view" => {
                    let db_todo: Vec<Todo> = pool.prep_exec("SELECT * from todo_list", ())
                        .map(|result| {
                            result
                                .map(|x| x.unwrap())
                                .map(|row| {
                                    let (id, desc, action) = my::from_row(row);
                                    Todo {
                                        id: id,
                                        desc: desc,
                                        action: action,
                                    }
                                })
                                .collect()
                        })
                        .unwrap();
                    print_todo(db_todo);
                }
                _ => println!("no argument passed"),
            }
        }
        3 => {
            let cmd = &args[1];
            let item = &args[2];
            match &cmd[..] {
                "add" => {
                    pool.prep_exec(r#"INSERT INTO `rust_todo`.`todo_list` (`desc`, `action`) VALUES (:desc, '0');"#, params!{"desc" => &item}).unwrap();
                },
                "complete" => {
                    pool.prep_exec(r#"UPDATE todo_list SET action = 1 WHERE id = :id;"#, params!{"id" => &item}).unwrap();
                },
                "remove" => {
                    pool.prep_exec(r#"DELETE FROM todo_list WHERE id = :id;"#, params!{"id" => &item}).unwrap();
                },
                _ => {
                    eprintln!("error: invalid command");
                }
            }
        }
        _ => println!("add, complete, and remove require 2 arguments"),
    }
}
