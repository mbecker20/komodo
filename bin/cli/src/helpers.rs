use std::io::Read;

use anyhow::Context;
use colored::Colorize;

pub fn wait_for_enter(press_enter_to: &str) -> anyhow::Result<()> {
  println!(
    "\nPress {} to {}\n",
    "ENTER".green(),
    press_enter_to.bold()
  );
  let buffer = &mut [0u8];
  std::io::stdin()
    .read_exact(buffer)
    .context("failed to read ENTER")?;
  Ok(())
}
