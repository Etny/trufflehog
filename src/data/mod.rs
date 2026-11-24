use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Result;

#[derive(Debug, Clone, Copy, Default)]
pub struct DistEntry {
    pub dist: u16,
    pub time: u16,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Frequency {
    #[default]
    One,
    Two,
    Three,
    Four,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Order {
    pub id: u16,
    pub freq: Frequency,
    pub containers: u8,
    pub volume: u16,
    pub time: u16,
    pub location: usize,
}

pub type Orders = [Order; 1177];
pub type Distances = [[DistEntry; 1099]; 1099];

pub fn load_distances() -> Result<Box<Distances>> {
    // We have to allocate a Vec first, to prevent a HUGE stackalloc (thanks clippy!)
    let vec_alloc: Vec<DistEntry> = vec![DistEntry::default(); 1099 * 1099];
    let box_slice = vec_alloc.into_boxed_slice();
    // EVIL STUPIC CAST!!
    let ptr = Box::into_raw(box_slice) as *mut Distances;
    let mut dist_box = unsafe { Box::from_raw(ptr) };

    let file = File::open("src/data/distances.txt")?;
    let lines = BufReader::new(file).lines();

    let mut buf = [0u16; 4];
    for line_res in lines {
        let line = line_res?;
        let nums = line.split(";").map(|x| x.parse::<u16>());

        for (i, num) in nums.enumerate() {
            buf[i] = num?
        }

        dist_box[buf[0] as usize][buf[1] as usize] = DistEntry {
            dist: buf[2],
            time: buf[3],
        };
    }

    Ok(dist_box)
}

fn freq_from_str(s: &str) -> Result<Frequency> {
    match s.trim() {
        "1PWK" => Ok(Frequency::One),
        "2PWK" => Ok(Frequency::Two),
        "3PWK" => Ok(Frequency::Three),
        "4PWK" => Ok(Frequency::Four),
        _ => anyhow::bail!("Failed to match"),
    }
}

// DONT FORGET: We are adding 18 so MILHEEZE has 0 time (instead of -0.18)
fn parse_decimal(s: &str) -> Result<u16> {
    // Specific exception for MILHEEZE
    if s.starts_with('-') {
        return Ok(0);
    }
    let mut parts = s.split(".");
    let left: u16 = parts.next().unwrap().parse()?;
    let right_str = parts.next().unwrap_or("0");
    let right: u16 = right_str.parse()?;

    if right > 99 {
        anyhow::bail!("Can't parse decimal")
    }

    let pow = 2 - right_str.len() as u32;
    let shift = u16::pow(10, pow);
    Ok((left * 100) + (right * shift) + 18)
}

pub fn load_orders() -> Result<Box<Orders>> {
    let mut order_box = Box::new([Order::default(); 1177]);

    let file = File::open("src/data/orders.txt")?;
    let lines = BufReader::new(file).lines();

    for (i, line_res) in lines.enumerate() {
        let line = line_res?;
        let mut parts = line.split(";");

        let id: u16 = parts.next().unwrap().parse()?;
        parts.next().unwrap(); // Skip city
        let freq = freq_from_str(parts.next().unwrap())?;
        let containers: u8 = parts.next().unwrap().parse()?;
        let volume: u16 = parts.next().unwrap().parse()?;
        let time: u16 = parse_decimal(parts.next().unwrap())?;
        let location: usize = parts.next().unwrap().parse()?;
        // Skip the X and Y coord

        order_box[i] = Order {
            id,
            freq,
            containers,
            volume,
            time,
            location,
        };
    }

    Ok(order_box)
}
