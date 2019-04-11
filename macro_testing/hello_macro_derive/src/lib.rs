extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}
fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

	let field_data = &ast.data;
	let field_types = match field_data {
		syn::Data::Struct(s) => Some(s.fields.to_owned()),
		syn::Data::Enum(_)=> None,
		syn::Data::Union(_) => None
	};
	let field_names = match field_types.unwrap() {
		syn::Fields::Named(n) => Some(n.named),
		syn::Fields::Unnamed(_) => None,
		syn::Fields::Unit => None
	};
	let field_name_option_vec:Option<Vec<syn::Ident>> = field_names.unwrap().into_pairs()
		.map(|pair| pair.value().ident.to_owned()).collect();	
	let mut field_name_vec = field_name_option_vec.unwrap();
	let first_field = &field_name_vec[0];
	
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro(&self) {
                println!("Hello, my struct name is {}, my first field is {} whos value is {}"
				, stringify!(#name),stringify!(#first_field),&self.#first_field);
            }
        }
    };
    gen.into()
}