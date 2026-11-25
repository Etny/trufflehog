use std::{arch::x86_64::_mm512_maskz_range_round_ps, default};

use crate::data::{ORDER_COUNT, get_dist, get_order};

pub struct Solution {
    pathA: WagonPath,
    pathB: WagonPath,
    pub cost: u64,
}

#[derive(Default)]
struct WagonPath {
    routes: [Vec<usize>; 5],
}

impl Solution {
    pub fn new_empty() -> Self {
        let mut empty = Self {
            pathA: WagonPath::new_empty(),
            pathB: WagonPath::new_empty(),
            cost: 0,
        };

        empty.pathA.routes[0][0] = 3;
        empty.pathA.routes[0].push(1);
        empty.pathA.routes[0].push(3);
        empty.pathA.routes[0].push(0);
        empty
    }

    pub fn random() -> Self {
        Self {
            pathA: WagonPath::random(),
            pathB: WagonPath::random(),
            cost: 0,
        }
    }

    pub fn calculate_score(&mut self) {
        let mut visit_count: [(u8, u8); ORDER_COUNT] = [(0, 0); ORDER_COUNT];
        let mut travel_time = 0;
        self.pathA.run_costs(&mut travel_time, &mut visit_count);
        self.pathB.run_costs(&mut travel_time, &mut visit_count);

        println!("Travel time: {}", travel_time);
        let mut penalty: u64 = 0;

        let mut unserviced = 0;
        for idx in 1..ORDER_COUNT {
            let order = get_order(idx);
            let pattern = visit_count[idx];
            let serviced = pattern.0 != 0xff && pattern.1 >= order.freq;
            if !serviced {
                unserviced += 1;
                penalty += (order.time * order.freq as u16 * 3) as u64;
            }
        }

        println!("unserviced: {}", unserviced);
        println!("Penalty: {}", penalty);
        self.cost = travel_time + penalty;
    }
}

fn mark_day(day: usize, pattern: &mut (u8, u8)) {
    if pattern.0 & (1 << day) != 0 {
        pattern.0 = 0xff;
    } else {
        pattern.0 |= 1 << day;
    }

    pattern.1 += 1;
}

impl WagonPath {
    pub fn run_costs(&self, travel_time: &mut u64, visit_count: &mut [(u8, u8); ORDER_COUNT]) {
        for day in 0..5 {
            let route = &self.routes[day];
            let mut last_loc = 287;
            for idx in route {
                let order = get_order(*idx);
                mark_day(day, &mut visit_count[*idx]);
                let travel_cost = get_dist(last_loc, order.location);
                let total_cost = (travel_cost + order.time) as u64;
                *travel_time += total_cost;
                last_loc = order.location;
            }
            *travel_time += 30 * 60;
        }
    }

    pub fn new_empty() -> Self {
        let mut routes: [Vec<usize>; 5] = Default::default();

        for route in &mut routes {
            route.push(0);
        }

        Self { routes }
    }

    pub fn random() -> Self {
        let mut routes: [Vec<usize>; 5] = Default::default();

        for route in &mut routes {
            let mut budget = 41400;
            let mut room: u32 = 100_000;
            let mut last_loc = 287;

            loop {
                let idx = fastrand::usize(0..ORDER_COUNT);
                let order = get_order(idx);
                let travel_cost = get_dist(last_loc, order.location);
                let total_cost = (travel_cost + order.time) as u32;
                let to_base_cost = get_dist(order.location, 0) as u32;

                if budget < total_cost + to_base_cost {
                    break;
                }
                if room < (order.containers as u16 * order.volume) as u32 {
                    budget -= get_dist(last_loc, 0) as u32;
                    if budget > 30 * 60 {
                        route.push(0);
                        room = 100_000;
                        budget -= 30 * 60;
                    } else {
                        break;
                    }
                } else {
                    budget -= total_cost;
                    room -= (order.containers as u16 * order.volume) as u32;
                    last_loc = order.location;
                    route.push(idx);
                }
            }

            route.push(0);
        }

        Self { routes }
    }

    pub fn write_route(&self, wagon_id: u8, builder: &mut String) {
        for day in 0..5 {
            for (i, node) in self.routes[day].iter().enumerate() {
                builder.push_str(&format!(
                    "{};{};{};{}\n",
                    wagon_id,
                    day + 1,
                    i + 1,
                    get_order(*node).id
                ));
            }
        }
    }
}

impl From<&Solution> for String {
    fn from(value: &Solution) -> Self {
        let mut builder = String::new();
        value.pathA.write_route(1, &mut builder);
        value.pathB.write_route(2, &mut builder);
        builder
    }
}
