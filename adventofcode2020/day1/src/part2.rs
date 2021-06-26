use itertools::Itertools;

use crate::helper;

pub fn solve_part2() -> Result<u32, String> {
    let input = match helper::load_input() {
        Ok(i) => i,
        Err(e) => return Err(e.to_string()),
    };

    let combo = input
        .iter()
        .copied()
        .combinations(3)
        .find(|x| x.iter().sum::<u32>() == 2020)
        .ok_or("Error: Combination not found")?;

    Ok(combo.iter().product())
}
