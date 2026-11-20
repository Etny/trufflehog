use crate::solution::{Neighbor, Neighborhood, Solution};

pub struct Solver<S: Solution> {
    temp: f32,
    _foo: std::marker::PhantomData<S>,
}

impl<S: Solution> Solver<S> {
    pub fn new() -> Self {
        Self {
            temp: 0.95,
            _foo: std::marker::PhantomData,
        }
    }

    pub fn run<const N: usize>(
        &mut self,
        ctx: &S::Context,
        neighborhoods: [&dyn Neighborhood<S>; N],
    ) -> S {
        let mut best = S::random();
        let mut current = best.clone();

        const ITERS: usize = 1_000_000;
        const TEMP_DROP: f32 = 0.02;
        loop {
            for _ in 0..ITERS {
                'n: for n in neighborhoods {
                    let per = n.perturb(&current);
                    if self.accept(per.error_delta(&current, ctx)) {
                        // let temp = current.clone();
                        current = current.apply(per);
                        // println!("Accepted {:?} over {:?}", temp, current);
                        if current.error(ctx) < best.error(ctx) {
                            best = current.clone()
                        }
                        break 'n;
                    }
                }
            }
            self.temp -= TEMP_DROP;
            if self.temp < 0.0 {
                break;
            }
        }

        best
    }

    fn accept(&self, delta_e: f32) -> bool {
        fastrand::f32() <= f32::exp(-delta_e / self.temp)
    }
}
