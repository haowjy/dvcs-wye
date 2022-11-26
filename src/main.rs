mod ui;

mod dsr;

mod cmd_function;

mod cmd_interface;
mod vc;

pub use cmd_interface::{createonly, readwrite, readonly};

fn main() {
    //gui_ui::UserInterface::gui();
    ui::UserInterface::receive_input_command_loop();
    //ui::UserInterface::<&str>::receive_input_command_test_inside();
}
