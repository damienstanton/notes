use anyhow::Result;
use libnotes::selfref::read;

fn main() -> Result<()> {
    read()?;
    Ok(())
}
