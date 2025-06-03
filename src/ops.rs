pub trait Sum {
    type Output;

    fn sum(self) -> Self::Output;
}

pub trait Dot<Rhs = Self> {
    type Output;

    fn dot(self, rhs: Rhs) -> Self::Output;
}
