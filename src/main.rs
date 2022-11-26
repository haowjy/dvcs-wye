mod ui;

mod dsr;

mod cmd_function;

mod cmd_interface;
mod vc;
mod gui_ui;

pub use cmd_interface::{createonly, readwrite, readonly};

fn main() {
    //gui_ui::UserInterface::gui();
    ui::UserInterface::<&str>::receive_input_command_loop();
    //ui::UserInterface::<&str>::receive_input_command_test_inside();
}
