mod ui;

mod dsr;

mod cmd_function;

mod cmd_interface;
pub use cmd_interface::{createonly, readwrite, readonly};

fn main() {
    println!("Hello, world!");
    ui::UserInterface::<&str>::receive_input_command_test_inside();
}
