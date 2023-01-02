mod app;
mod crossterm;
mod ui;

use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

use anyhow::anyhow;

use copypasta_ext::prelude::*;
use copypasta_ext::x11_fork::ClipboardContext;

pub fn test() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut ctx = ClipboardContext::new()?; 
    println!("clipboard: {:?}", ctx.get_contents()?);
    ctx.set_contents("hello".into())?;
    ctx = ClipboardContext::new()?;
    println!("clipboard: {:?}", ctx.get_contents()?);
    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    const HOME_VAR: &str = "HOME";
    const CLIPSTER_DIR: &str = ".clipster";
    const TICK_RATE: Duration = Duration::from_millis(200);

    // Setup ~/.clipster/
    let home: String =
        env::var(HOME_VAR).map_err(|err| anyhow!(HOME_VAR.to_owned() + " " + &err.to_string()))?;
    let mut clipster_path = PathBuf::from(&home);
    clipster_path.push(CLIPSTER_DIR);
    if !clipster_path.as_path().is_dir() {
        fs::create_dir_all(clipster_path.as_path())?;
    }

    test().map_err(|err| anyhow!(err.to_string()))?;

    crossterm::run(TICK_RATE)?;
    println!("hello world");
    Ok(())
}
