use async_std::stream::IntoStream;
use orm::ToTable;
use orm_setup;
use sqlx::migrate::Migrate;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{ConnectOptions, Connection, Database, Executor, SqliteConnection};
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::future::Future;
use std::ops::DerefMut;
use std::str::FromStr;
use std::sync::Mutex;
//use rusqlite::{params, Connection, Result};

orm_setup::setup_all!(sqlite, file "db.sqlite");

#[derive(Debug, ToTable, sqlx::FromRow)]
struct Pear {
    id: i32,
}

#[derive(Debug, ToTable, sqlx::FromRow)]
struct Person {
    id: i32,
    id2: i32,
    name: String,
}


fn main() -> Result<(), sqlx::Error> {


    //let init = dbg!(Person::table_init_stmt());
    //let mut c =  *CONNECTION.lock().unwrap();

    //sqlx::query(&init).fetch_all(&mut *CONNECTION.lock().unwrap()).await?;
    Ok(())
}
pub fn test_table() {
    println!("{}", Pear::table_init_stmt());
}
pub fn test_insert() {
    let p = Person {
        id: 10,
        id2: 11,
        name: "JAn".to_string(),
    };
    println!("{}", p.insert_stmt());
}
#[cfg(test)]
mod tests {

    use crate::{main, test_insert, test_table, Person};

    #[test]
    fn table_works() {
        test_table();
    }

    #[test]
    fn main_works2() {
        match main() {
            Err(a) => print!("{:?}", a),
            _ => print!("ALL WORKED"),
        }
    }

    #[test]
    fn insert_works() {
        test_insert();
    }
}
