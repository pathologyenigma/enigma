use enigma::app::{Application, Result, Runnable, Runner};
struct Viewer;

impl Runnable for Viewer {}

fn main() -> Result<()> {
    Application::build().run(Viewer {})?;
    Ok(())
}
