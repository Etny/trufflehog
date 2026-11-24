mod consts;
mod data;

use anyhow::Result;

fn main() -> Result<()> {
    // Print some art :)
    println!("{}", consts::HOG_ART);
    println!("Welcome to trufflehog!");

    let orders = data::load_orders()?;

    println!("{:?}", orders.iter().map(|o| o.id).max().unwrap());

    Ok(())
}
