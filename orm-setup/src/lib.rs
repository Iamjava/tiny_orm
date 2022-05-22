extern crate core;
extern crate proc_macro;
extern crate proc_macro2;
extern crate sqlx;

#[macro_export]
macro_rules! setup_all {
    // `()` indicates that the macro takes no argument.
    (sqlite, file $filename:expr) => {
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
       let database_file = $filename;
        let database_url = format!("sqlite://{}", database_file);
         let mut conn = async_std::task::block_on(async {
          SqliteConnectOptions::from_str(&database_url).unwrap().create_if_missing(true).connect().await.unwrap()
            });
        dbg!(Mutex::new(conn))
    };
}

    };
}
