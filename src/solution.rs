pub trait Solution: Clone + std::fmt::Debug {
    type Context;
    type Mutation: Neighbor<Self>;

    fn error(&self, ctx: &Self::Context) -> f32;
    fn apply(self, delta: Self::Mutation) -> Self;
    fn random() -> Self;
}

pub trait Neighbor<S: Solution> {
    fn error_delta(&self, sol: &S, ctx: &S::Context) -> f32;
}

pub trait Neighborhood<S: Solution> {
    fn perturb(&self, sol: &S) -> S::Mutation;
}

impl<S, T> Neighborhood<S> for T
where
    S: Solution,
    T: Fn(&S) -> S::Mutation,
{
    fn perturb(&self, sol: &S) -> <S as Solution>::Mutation {
        self(sol)
    }
}
