use crate::{
    solution::{Neighbor, Solution},
    solver::Solver,
};

mod hog;
mod solution;
mod solver;

impl solution::Solution for f32 {
    type Context = ();
    type Mutation = f32;

    fn error(&self, _: &Self::Context) -> f32 {
        f32::powi(*self, 4) - (4.0 * f32::powi(*self, 2)) + (0.5 * *self)
    }

    fn apply(self, delta: Self::Mutation) -> Self {
        delta
    }

    fn random() -> Self {
        (fastrand::f32() - 0.5) * 20.0
    }
}

impl Neighbor<f32> for f32 {
    fn error_delta(&self, sol: &f32, ctx: &<f32 as solution::Solution>::Context) -> f32 {
        self.error(ctx) - sol.error(ctx)
    }
}

fn add_random(sol: &f32) -> f32 {
    *sol + ((fastrand::f32() - 0.5) * 0.1)
}

fn main() {
    // Print some art :)
    println!("{}", hog::HOG_ART);
    println!("Welcome to trufflehog!");

    // We maken een nieuw solver...
    let mut solver: Solver<f32> = solver::Solver::new();
    // ...en pakken de oplossing
    let solution = solver.run(&(), [&add_random]);

    // Dit is hoe je waardes print: println!
    // vervangt de n-de '{}' in de string door
    // het n-de argument dat je meegeeft na
    // de string
    println!("Found solution: {}", solution);
}
