use std::process::ExitCode;

pub mod utils;
pub mod raycaster;
pub mod rendering;
pub mod ui;
pub mod behaviours;
pub mod app;

#[cfg(test)]
pub mod tests;

fn main() -> ExitCode {
    match app::App::init() {
        Ok(mut app) => {
            app.run();
            ExitCode::SUCCESS
        },
        Err(err) => {
            eprintln!("{}", err);
            ExitCode::FAILURE
        }
    }
}
