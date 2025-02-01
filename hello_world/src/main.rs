mod greeting;
use hello_world_lib::greeting_from_lib;
use greeting::{default_greeting,french,spanish};

fn main (){
println!("Hello, world!");
println!("{}", default_greeting());
println!("{}", spanish::default_greeting());
println!("{}", french::default_greeting());
println!("{}", hello_world_lib::greeting_from_lib())

}

