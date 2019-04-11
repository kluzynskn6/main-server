use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes{
	flavor:String,
	quantity: u32,
}

fn main() {
	let chocolate_pancake = Pancakes{
		flavor: "Chocolate".to_string(),
		quantity: 11,
	};
	
    chocolate_pancake.hello_macro();
}