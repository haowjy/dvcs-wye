mod ui;

mod dsr;

mod cmd_function;

mod cmd_interface;
mod vc;

pub use cmd_interface::{createonly, readwrite, readonly};

fn main() {
    let _res=ui::Wye::input_command();
}
