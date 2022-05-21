use std::borrow::BorrowMut;
use std::future::Future;
use std::ops::DerefMut;
use std::str::FromStr;
use async_std::stream::IntoStream;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{Connection, ConnectOptions, Database, Executor, SqliteConnection};
use orm::ToTable;
use std::collections::HashMap;
use std::sync::Mutex;
use sqlx::migrate::Migrate;
//use rusqlite::{params, Connection, Result};

macro_rules! setup_all {
    // `()` indicates that the macro takes no argument.
    () => {
      pub trait ToTable {
    fn table_init_stmt() ->String;
    fn insert_stmt(&self)->String;
    fn delete_all();

    fn init();
    fn get_all()->Vec<Self> where  Self:Sized;
    fn insert(&self);
    }


        lazy_static::lazy_static! {
    static ref CONNECTION: Mutex<SqliteConnection> = {
       let database_file = "db.sqlite";
        let database_url = format!("sqlite://{}", database_file);
         let mut conn = async_std::task::block_on(async {
          SqliteConnectOptions::from_str(&database_url).unwrap().create_if_missing(true).connect().await.unwrap()
            });
        dbg!(Mutex::new(conn))
    };
}

    };
}

setup_all!();

#[derive(Debug, ToTable)]
#[derive(sqlx::FromRow)]
struct Pear {
    id: i32,
}



#[derive(Debug, ToTable)]
#[derive(sqlx::FromRow)]
struct Person {
    id: i32,
    id2: i32,
    name: String,
}


#[async_std::main]
async fn main()->Result<(),sqlx::Error> {
    let database_file = "db.sqlite";
    let database_url = format!("sqlite://{}", database_file);
    let mut conn = SqliteConnectOptions::from_str(&database_url)?
        .create_if_missing(true).connect().await?;


    //let init = dbg!(Person::table_init_stmt());
    //let mut c =  *CONNECTION.lock().unwrap();

    //sqlx::query(&init).fetch_all(&mut *CONNECTION.lock().unwrap()).await?;



    let p = Person {
        id: 111,
        id2: 111,
        name: "JAnnn".to_string(),
    };
    let p2 = Person {
        id: 11,
        id2: 12,
        name: "Jaaaan".to_string(),
    };

    println!("{}", p.insert_stmt());
    p.insert();
    p2.insert();
    Person::delete_all();
    dbg!(Person::get_all());
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

    use crate::{test_insert, test_table, Person, main};

    #[test]
    fn table_works() {
        test_table();
    }

    #[test]
    fn main_works2(){ match main() {
        Err(a)=> print!("{:?}",a),
        _ => print!("ALL WORKED")
    } }

    #[test]
    fn insert_works() {
        test_insert();
    }
}
