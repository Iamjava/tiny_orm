extern crate proc_macro;
extern crate proc_macro2;
extern crate core;

use std::string::String;
use proc_macro::{TokenStream};
use std::any::Any;
use std::fmt::format;
use quote::quote;
use syn::{Item, Fields, Type, Ident};



#[proc_macro_derive(ToTable)]
pub fn hello_macro_derive(input: TokenStream)->TokenStream{
    let item: syn::ItemStruct = syn::parse(input.clone()).expect("failed to parse input");
    let vars = get_names_and_types(&item);
    let create_stmt = generate_create_stmt(vars.clone()); //TODO make vars a Reference
    let name = &item.ident;
    let insert_stmt = generate_insert_stmt(vars,name);
    let sin: proc_macro2::TokenStream = insert_stmt.parse().unwrap();

    let gen = quote!{
        impl ToTable for #name {
            fn table_init()->String{
                format!("CREATE TABLE IF NOT EXISTS {} ({});",stringify!(#name),#create_stmt)
            }

            fn insert(&self)->String{
              //println!("{}",#insert_stmt);
               #sin
            }

           fn get_all<E>(con: &mut E)->Vec<#name> where
                 Self:Sized,
                 E: 'e + Executor<'c, Database = SqliteConnection>{
                //async{
               //   return sqlx::query_as::<_, Person>("SELECT * FROM Person").fetch_all(&mut con).await?;
                //}
                vec![]
            }

        }
    };
    gen.into()
}


fn generate_create_stmt(vars : Vec<(String,String)>) -> String {
    vars.into_iter().map(|(name,sqlType)| format!("{name} {sqlType}")).collect::<Vec<String>>().join(",")
}

fn generate_insert_stmt(vars : Vec<(String,String)>,name: &Ident) -> String {
    let values = vars.iter().map(|(name,_)|format!("self.{name}")).collect::<Vec<String>>().join(",");
    //let values = format!(", {values}");
    let brackets = vec!["'{}'";vars.len()].join(",");
    let name_str = quote!{#name}.to_string();
    let s= format!(r##"format!("INSERT INTO {name_str} VALUES ({brackets})",{})"##,values);
    println!("{}", s);
    return  s
}

fn to_sql_types(inputType: String,name: &str) -> String{
    match inputType.as_str() {

        "String" => "varchar(1024)".to_string(),
        "i8"|"i32"|"i64"|"i128" => "INT".to_string(),

        _ => panic!("UNKNOWN TYPE, Cannot Convert to SQL type {}", inputType)
    }
}

fn impl_hello(ast: &syn::DeriveInput)->TokenStream{
    println!("invoked");
    let name = &ast.ident;
    let gen = quote!{
        impl ToTable for #name {
            fn table_init(){
                println!("CREATE TABLE IF NOT EXISTS {} ",stringify!(#name));
            }
        }
    };
    gen.into()
}

fn get_names_and_types(struct_: &syn::ItemStruct) -> Vec<(String, String)> {
    match struct_.fields {
        // A field can only be named "bees" if it has a name, so we'll
        // match those fields and ignore the rest.
        Fields::Named(ref fields) => {
            // Unwrap the field names because we know these are named fields.
            let fields_typed = fields.named.iter().map(
                |field|(
                    field.ident.as_ref().unwrap().to_string(),
                    {let t = &field.ty;  TokenStream::from(quote! {#t}).to_string()}
                )
            );
            //let a = &fields_typed.collect::<Vec<(String,String)>>()[1];
            fields_typed.map(|(name,inputType)| (name.clone(),to_sql_types(inputType,&name))).collect()

        }
        // Ignore unit structs or anonymous fields.
        _ => {
            vec![]
        },
    }
}