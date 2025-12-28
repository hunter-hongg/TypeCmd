use typecmd::prelude::*;

fn main() -> Result<()> {
    let mut typecmd = TypeCmd::new()?;
    typecmd.run()
}