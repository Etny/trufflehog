use crate::solution::{Neighbor, Neighborhood, Solution};

pub struct Solver<S: Solution> {
    temp: f32,
    _foo: std::marker::PhantomData<S>,
}

impl<S: Solution> Solver<S> {
    pub fn new() -> Self {
        Self {
            temp: 0.99,
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

        const ITERS: usize = 10_000_000;
        const TEMP_DROP: f32 = 0.01;
        while self.temp > f32::EPSILON {
            for _ in 0..ITERS {
                'n: for n in neighborhoods {
                    let per = n.perturb(&current);
                    if self.accept(per.error_delta(&current, ctx)) {
                        current = current.apply(per);
                        if current.error(ctx) < best.error(ctx) {
                            best = current.clone()
                        }
                        break 'n;
                    }
                }
            }
            self.temp -= TEMP_DROP
        }

        best
    }

    fn accept(&self, delta_e: f32) -> bool {
        fastrand::f32() <= f32::exp(delta_e / self.temp)
    }
}
