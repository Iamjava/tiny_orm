
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{ConnectOptions, SqliteConnection};
use std::str::FromStr;
use std::sync::Mutex;
//use rusqlite::{params, Connection, Result};

orm_setup::setup_all!(sqlite, file "db.sqlite");

#[derive(Debug, orm::ToTable, sqlx::FromRow)]
struct Pear{
    id: i32,
    age: i32,
    producer: String,
}

#[derive(Debug, orm::ToTable, sqlx::FromRow)]
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
pub fn test_filter() {
    Pear::init(); //Initalisiert die Tabelle Falls nicht Vorhanden
    Pear::delete_all();
    let pear = Pear{
        id: 1,
        age: 2,
        producer: "Lady Bird".to_string(),
    };
    let pear2 = Pear{
        id: 100,
        age: 2,
        producer: "Lady Bird".to_string(),
    };
    let pear3 = Pear{
        id: 101,
        age: 4,
        producer: "Lady Bird2".to_string(),
    };

    pear.save();
    pear2.save();
    pear3.save();
    assert_eq!(Pear::get_all_filter("id < 100").len(),1);// -> gibt all p's in der Datenbank wieder zurück
    assert_eq!(Pear::get_all_filter("age = 2").len(),2);
    Pear::delete_all();
    assert_eq!(dbg!(Pear::get_all().len()),0);
}
pub fn test_insert() {
    Pear::init(); //Initalisiert die Tabelle Falls nicht Vorhanden
    Pear::delete_all();
    let pear = Pear{
        id: 1,
        age: 2,
        producer: "Lady Bird".to_string(),
    };

    pear.save(); //Speichert in die Aktuelle Tabelle
    assert_eq!(Pear::get_all().len(),1);// -> gibt all p's in der Datenbank wieder zurück
    Pear::delete_all();
}
#[cfg(test)]
mod tests {

    use crate::{main, test_insert, test_filter};

    #[test]
    fn table_works() {
        test_filter();
    }

    #[test]
    fn insert_works() {
        test_insert();
    }
}
