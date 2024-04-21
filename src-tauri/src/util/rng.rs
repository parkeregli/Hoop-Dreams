use rand::distributions::{Distribution, WeightedIndex};
use rand::Rng;

pub fn rng(home_value: i32, away_value: i32) -> u8 {
    println!("Home: {} Away: {}", home_value, away_value);
    // 0 = home, 1 = away
    let sub = home_value - away_value;
    let abs_sub = sub.abs();
    let ratio = abs_sub + 50;
    let mut rng = rand::thread_rng();
    let winner = rng.gen_range(0..100);

    println!("Sub: {}", sub);
    println!("Ratio: {}", ratio);
    println!("Rand: {}", winner);

    if sub == 0 {
        if winner < 50 {
            0
        } else {
            1
        }
    } else if sub > 0 {
        if winner < ratio {
            0
        } else {
            1
        }
    } else {
        if winner < ratio {
            1
        } else {
            0
        }
    }
}
pub fn rng_arr(arr: Vec<i32>) -> u8 {
    let mut rng = rand::thread_rng();
    let weights: Vec<_> = arr.iter().map(|&v| v.max(0) as u64).collect();
    let dist = WeightedIndex::new(weights).unwrap();
    let winner = dist.sample(&mut rng);
    println!("Winner: {}", winner);
    winner as u8
}
