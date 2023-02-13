use anyhow::{anyhow, Result};
use which::which;

const DEPENDENCIES: [&str; 2] = ["meson", "ninja"];

fn main() -> Result<()> {
    for dependency in DEPENDENCIES {
        which(dependency).map_err(|err| anyhow!("{err}: dependency"))?;
    }
    Ok(())
}
