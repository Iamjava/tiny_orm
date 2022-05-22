# Tinyorm 
### Tinyorm is a sqlx-based minimal rust orm in development. 
Since I didnt find a proper Orm that worked for me and managed 
small web-app record like insertions in the DB i built tinyorm. 
It does currently not suport any fancy relations or that stuff, to be honest it only supports ints and Strings and inserts them into 
the DB. I will work on making it nicer to use and also it uses async code in the libs that is blocking due to rust not allowing 
`async` traits.

### a working example: 
```rust
orm_setup::setup_all!(sqlite, file "db.sqlite");

#[derive(Debug, orm::ToTable, sqlx::FromRow)]
struct Pear{
    id: i32,
    age: i32,
    producer: String,
}

fn example(){
    Pear::init(); //Initalisiert die Tabelle Falls nicht Vorhanden
    
    let pear = Pear{
        id: 1,
        age: 2,
        producer: "Lady Bird".to_string(),
    };
    
    pear.save(); //Speichert in die Aktuelle Tabelle
    dbg!(Pear::get_all());// -> gibt all p's in der Datenbank wieder zurück
}

```
