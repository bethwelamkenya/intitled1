use std::ptr::null;
use sqlite::State;
use sql::prelude::*;

fn database(){
}

fn create_member_table(){
    let connection = sqlite::open(":memory:").unwrap();

    let query = "
    CREATE TABLE users (name TEXT, age INTEGER);
    INSERT INTO users VALUES ('Alice', 42);
    INSERT INTO users VALUES ('Bob', 69);
    ";
    connection.execute(query).unwrap();

}

fn insert_member(name : String){
    let connection = sqlite::open(":memory:").unwrap();

}

fn get_member(name : String){
    let my_name: &str = &*name;
    let connection = sqlite::open(":memory:").unwrap();
    // let member = null();
    let query = "SELECT * FROM member WHERE name = ?";
    let mut statement = connection.prepare(query).unwrap();
    statement.bind((1, my_name)).unwrap();

    while let Ok(State::Row) = statement.next() {
        println!("name = {}", statement.read::<String, _>("name").unwrap());
        println!("age = {}", statement.read::<i64, _>("age").unwrap());
    }
}

fn get_member_cursor(name : String){
    let my_name = &*name;
    let connection = sqlite::open(":memory:").unwrap();
    let query = "SELECT * FROM member WHERE name = ?";

    for row in connection
        .prepare(query)
        .unwrap()
        .into_iter()
        .bind((1, my_name))
        .unwrap()
        .map(|row| row.unwrap())
    {
        println!("name = {}", row.read::<&str, _>("name"));
        println!("age = {}", row.read::<i64, _>("age"));
    }
}

fn example(){
    // CREATE TABLE `users` (`id` INTEGER NOT NULL, `name` TEXT, `photo` BLOB)
    println!("{}", create_table("users").column("id".integer().not_null())
        .column("name".string())
        .column("photo".binary())
        .compile().unwrap());

// DELETE FROM `users`
    println!("{}", delete_from("users").compile().unwrap());

// INSERT INTO `users` (`id`, `name`) VALUES (?, ?), (?, ?)
    println!("{}", insert_into("users").columns(&["id", "name"]).batch(2)
        .compile().unwrap());

// SELECT * FROM `users` WHERE `name` LIKE 'A%'
    println!("{}", select_from("users").so_that(column("name").like("A%"))
        .compile().unwrap());

// SELECT * FROM `users` ORDER BY `name` DESC
    println!("{}", select_from("users").order_by(column("name").descend())
        .compile().unwrap());

// SELECT `name`, `photo` FROM `users` LIMIT 1
    println!("{}", select_from("users").columns(&["name", "photo"]).limit(1)
        .compile().unwrap());
}