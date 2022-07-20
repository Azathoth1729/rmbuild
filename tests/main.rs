use std::path::PathBuf;

use anyhow::Result;
use home::home_dir;

use rmbuild::visit;

fn workshop() -> PathBuf {
    let mut home = home_dir().unwrap();
    home.push("workshop");
    home
}

#[test]
fn test_it_works() -> Result<()> {
    visit(workshop())?;
    Ok(())
}
