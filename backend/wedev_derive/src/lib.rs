extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;
use deisel::query_source::Table;
use diesel;
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

use crate::errors::WebdevError;
use crate::errors::WebdevErrorKind;

use crate::search::NullableSearch;
use crate::search::Search;

#[proc_macro_derive(GenericTableFunctions)]
pub fn generic_table_functions_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_table_functions(&ast)
}
fn impl_generic_table_functions(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl GenericTableFunctions for #name {
            fn get(id:u64,table:Table,con:&MysqlConnection) {
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
            fn delete (id:u64,table:Table,con:&MysqlConnection) {
            
            }
        }
    };
    gen.into()
}
