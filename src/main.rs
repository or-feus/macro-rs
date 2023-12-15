mod macros;

use macro_rs::log_function;

fn main() {
    my_function();
}

#[log_function]
fn my_function() {
    println!("hello my_function");
}
