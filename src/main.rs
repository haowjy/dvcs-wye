mod ui;

mod cmd_function;

mod cmd_interface;
pub use cmd_interface::{createonly, read_write,readonly};

fn main() {
    println!("Hello, world!");
}
