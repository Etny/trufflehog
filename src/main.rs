mod consts;
mod data;
mod solution;

use anyhow::Result;

use crate::solution::Solution;

fn main() -> Result<()> {
    // Print some art :)
    // println!("{}", consts::HOG_ART);
    // println!("Welcome to trufflehog!");

    data::load_distances()?;
    data::load_orders()?;

    let mut sol = Solution::random();
    let text: String = String::from(&sol);

    println!("{}", text);
    sol.calculate_score();
    println!("{}", sol.cost);

    println!("{}", data::get_dist(287, 321) + data::get_dist(321, 287));

    Ok(())
}
