extern crate proc_macro;
#[macro_use]
extern crate diesel;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;
use diesel::query_source::Table;
use diesel::mysql::Mysql;
use diesel::mysql::MysqlConnection;
use diesel::query_builder::AsQuery;
use diesel::query_builder::BoxedSelectStatement;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::TextExpressionMethods;

use log::trace;
use log::info;
use log::warn;
use log::error;

use backend_support::search::NullableSearch;
use backend_support::search::Search;

use backend_support::table_traits;
/* Alright, here's how macros work.
 *   When you call #derive(macro_name) it calls the corresponding
 *   public function marked with proc_macro_derive.
 *   This function turns the supplied struct into a variable and 
 *   passes it to the derive function.
 *
 *   The derive function can then get all information about the stuct and other inputs to the
 *   supplied function (i.e genericstruct.get(9) has the struct info, the type info, and the 9).
 *
 *   The function needs to be automatically built from just the struct type info. You can get the
 *   struct name, all field names, and pretty much anything else through the ast sent.
 *
 */
#[proc_macro_derive(GenericTableFunctions)]
pub fn generic_table_functions_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_generic_table_functions(&ast)
}

#[proc_macro_derive(CreateTableFunctions)]
pub fn create_table_functions_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_create_table_functions(&ast)
}
#[proc_macro_derive(UpdateTableFunctions)]
pub fn update_table_functions_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_update_table_functions(&ast)
}
#[proc_macro_derive(SearchTableFunctions)]
pub fn search_table_functions_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_search_table_functions(&ast)
}

fn impl_generic_table_functions(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl GenericTableFunctions for #name {
            fn get(&self, id:u64:&MysqlConnection) {
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
            fn delete (&self, id:u64,con:&MysqlConnection) {
            
            }
        }
    };
    gen.into()
}

fn impl_create_table_functions(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl CreateTableFunctions for #name {
            fn <T>create(&self,con:&MysqlConnection) {
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };
    gen.into()
}

fn impl_update_table_functions(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl UpdateTableFunctions for #name {
            fn update<T>(&self,con:&MysqlConnection) {
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };
    gen.into()
}

fn impl_search_table_functions(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl SearchTableFunctions for #name {
            fn search<T>(&self,table:Table,con:&MysqlConnection) {
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };
    gen.into()
}
