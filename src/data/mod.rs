use std::{
    fs::File,
    io::{BufRead, BufReader},
    sync::OnceLock,
};

use anyhow::Result;

static ORDERS: OnceLock<&'static Orders> = OnceLock::new();
static DISTANCES: OnceLock<&'static Distances> = OnceLock::new();

pub fn get_dist(start: usize, end: usize) -> u16 {
    all_distances()[start][end]
}

pub fn all_distances() -> &'static Distances {
    DISTANCES.get().unwrap()
}

pub fn get_order(idx: usize) -> &'static Order {
    &all_orders()[idx]
}

pub fn all_orders() -> &'static Orders {
    ORDERS.get().expect("Failed to get orders")
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Order {
    pub id: u16,
    pub freq: u8,
    pub containers: u8,
    pub volume: u16,
    pub time: u16,
    //pub time_f: f32,
    pub location: usize,
}

pub const ORDER_COUNT: usize = 1178;
pub type Orders = [Order; ORDER_COUNT];
pub type Distances = [[u16; 1099]; 1099];

pub fn load_distances() -> Result<()> {
    // We have to allocate a Vec first, to prevent a HUGE stackalloc (thanks clippy!)
    let vec_alloc: Vec<u16> = vec![0; 1099 * 1099];
    let box_slice = vec_alloc.into_boxed_slice();
    // EVIL STUPID CAST!!
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

        dist_box[buf[0] as usize][buf[1] as usize] = buf[3];
    }

    let leaked_distances = Box::leak(dist_box);
    DISTANCES.set(leaked_distances).unwrap();

    Ok(())
}

fn freq_from_str(s: &str) -> Result<u8> {
    match s.trim() {
        "1PWK" => Ok(1),
        "2PWK" => Ok(2),
        "3PWK" => Ok(3),
        "4PWK" => Ok(4),
        _ => anyhow::bail!("Failed to match"),
    }
}

// DONT FORGET: We are adding 18 so MILHEEZE has 0 time (instead of -0.18)
fn parse_decimal(s: &str) -> Result<u16> {
    let float_time: f32 = s.parse().unwrap();
    return Ok((float_time * 60.0) as u16);
}

pub fn load_orders() -> Result<()> {
    let mut order_box = Box::new([Order::default(); ORDER_COUNT]);

    let file = File::open("src/data/orders.txt")?;
    let lines = BufReader::new(file).lines();

    order_box[0] = Order {
        id: 0,
        freq: 0,
        containers: 0,
        volume: 0,
        time: 0,
        //time_f: 0.0,
        location: 287,
    };

    for (i, line_res) in lines.enumerate() {
        let line = line_res?;
        let mut parts = line.split(";");

        let id: u16 = parts.next().unwrap().parse()?;
        parts.next().unwrap(); // Skip city
        let freq = freq_from_str(parts.next().unwrap())?;
        let containers: u8 = parts.next().unwrap().parse()?;
        let volume: u16 = parts.next().unwrap().parse()?;
        let time_f: f32 = parts.next().unwrap().parse()?;
        let time: u16 = f32::round(time_f * 60.0) as u16;
        let location: usize = parts.next().unwrap().parse()?;
        // Skip the X and Y coord

        order_box[i + 1] = Order {
            id,
            freq,
            containers,
            volume,
            time,
            //time_f,
            location,
        };
    }

    let leaked_orders: &'static Orders = Box::leak(order_box);
    ORDERS.set(leaked_orders).unwrap();

    Ok(())
}
