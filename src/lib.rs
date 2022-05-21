use std::borrow::BorrowMut;
use std::ops::DerefMut;
use std::str::FromStr;
use async_std::stream::IntoStream;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{Connection, ConnectOptions, Database, Executor, SqliteConnection};
use orm::ToTable;

//use rusqlite::{params, Connection, Result};

pub trait ToTable {
    fn table_init()->String;
    fn insert(&self)->String;
    fn get_all<'e, 'c: 'e, E>( executor: E)->Vec<Self>
        where
        Self:Sized,
        E: 'e + Executor<'c, Database = SqliteConnection>;

}


#[derive(Debug, ToTable)]
#[derive(sqlx::FromRow)]
struct Person {
    id: i32,
    id2: i32,
    name: String,
}


fn get_all(con: &mut sqlx::SqliteConnection)->Vec<Person> {
//async{
//   return sqlx::query_as::<_, Person>("SELECT * FROM Person").fetch_all(&mut con).await?;
//}
return vec![]
}


#[async_std::main]
async fn main()->Result<(),sqlx::Error> {
    use sqlx::Connection;
    let database_file = "db.sqlite";
    let database_url = format!("sqlite://{}", database_file);
    let mut conn = SqliteConnectOptions::from_str(&database_url)?
        .create_if_missing(true).connect().await?;
    let init = dbg!(Person::table_init());
    let a = sqlx::query(&init).fetch_all(&mut conn).await?;

    let p = Person {
        id: 10,
        id2: 11,
        name: "JAn".to_string(),
    };
    let ins = dbg!(p.insert());
    println!("{}", p.insert());
    sqlx::query(&ins).fetch_all(&mut conn).await?;
    let mut stream = sqlx::query_as::<_, Person>("SELECT * FROM Person").fetch_all(&mut conn).await?;
    let m = sqlx::query("DELETE FROM Person").fetch_all(&mut conn).await?;
    println!("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa {:?}", stream);
    println!("OK");
    Ok(())
}
pub fn test_table() {
    println!("{}", Person::table_init());
}
pub fn test_insert() {
    let p = Person {
        id: 10,
        id2: 11,
        name: "JAn".to_string(),
    };
    println!("{}", p.insert());
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
        _ => print!("OKII")
    } }

    #[test]
    fn insert_works() {
        test_insert();
    }
}
