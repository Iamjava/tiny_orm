extern crate core;
extern crate proc_macro;
extern crate proc_macro2;
use proc_macro::TokenStream;
use quote::{quote};
use std::string::String;
use syn::{Fields, Ident,};

#[proc_macro_derive(ToTable)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let  item: syn::ItemStruct = syn::parse(
        format!(
            "\n #[derive(sqlx::FromRow)] \n {}",
            input.clone().to_string()
        )
        .parse()
        .unwrap(),
    )
    .expect("failed to parse input");
    let vars = get_names_and_types(&item);
    let create_stmt = generate_create_stmt(vars.clone()); //TODO make vars a Reference
    let name = &item.ident;
    let insert_stmt = generate_insert_stmt(vars, name);
    let sin: proc_macro2::TokenStream = insert_stmt.parse().unwrap();

    let gen = quote! {
        impl ToTable for #name {
            fn table_init_stmt()->String{
                format!("CREATE TABLE IF NOT EXISTS {} ({});",stringify!(#name),#create_stmt)
            }

            fn init(){
                 async_std::task::block_on(async {
                  sqlx::query(&#name::table_init_stmt()).fetch_all(&mut *CONNECTION.lock().unwrap()).await.unwrap();
                 });
            }

            fn delete_all(){
                use async_std::task;
                task::block_on(async {
                  sqlx::query(&format!("DELETE FROM {}",stringify!(#name))).fetch_all(&mut *CONNECTION.lock().unwrap()).await.unwrap();
                 });
            }

            fn insert_stmt(&self)->String{
              //println!("{}",#insert_stmt);
               #sin
            }


            fn insert(&self){
                use async_std::task;
                task::block_on(async {
                  sqlx::query(&self.insert_stmt()).fetch_all(&mut *CONNECTION.lock().unwrap()).await.unwrap();
                 });
            }

            fn get_all() -> Vec<#name> {
                use async_std::task;
                return task::block_on(async {
                  sqlx::query_as::<_, #name>("SELECT * FROM Person").fetch_all(&mut *CONNECTION.lock().unwrap()).await.unwrap()
                 });
            }

        }
    };
    gen.into()
}

fn generate_create_stmt(vars: Vec<(String, String)>) -> String {
    vars.into_iter()
        .map(|(name, sql_type)| format!("{name} {sql_type}"))
        .collect::<Vec<String>>()
        .join(",")
}

fn generate_insert_stmt(vars: Vec<(String, String)>, name: &Ident) -> String {
    let values = vars
        .iter()
        .map(|(name, _)| format!("self.{name}"))
        .collect::<Vec<String>>()
        .join(",");
    //let values = format!(", {values}");
    let brackets = vec!["'{}'"; vars.len()].join(",");
    let name_str = quote! {#name}.to_string();
    let s = format!(
        r##"format!("INSERT INTO {name_str} VALUES ({brackets})",{})"##,
        values
    );
    println!("{}", s);
    return s;
}

fn to_sql_types(input_type: String, _name: &str) -> String {
    match input_type.as_str() {
        "String" => "varchar(1024)".to_string(),
        "i8" | "i32" | "i64" | "i128" => "INT".to_string(),

        _ => panic!("UNKNOWN TYPE, Cannot Convert to SQL type {}", input_type),
    }
}


fn get_names_and_types(struct_: &syn::ItemStruct) -> Vec<(String, String)> {
    match struct_.fields {
        // A field can only be named "bees" if it has a name, so we'll
        // match those fields and ignore the rest.
        Fields::Named(ref fields) => {
            // Unwrap the field names because we know these are named fields.
            let fields_typed = fields
                .named
                .iter()
                // .filter(|field| field.ident.as_ref().unwrap() != connection_name)
                .map(|field| {
                    (field.ident.as_ref().unwrap().to_string(), {
                        let t = &field.ty;
                        TokenStream::from(quote! {#t}).to_string()
                    })
                });
            //let a = &fields_typed.collect::<Vec<(String,String)>>()[1];
            fields_typed
                .map(|(name, input_type)| (name.clone(), to_sql_types(input_type, &name)))
                .collect()
        }
        // Ignore unit structs or anonymous fields.
        _ => {
            vec![]
        }
    }
}
