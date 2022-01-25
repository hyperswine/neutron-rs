// std OPS for kernel, add, sub, etc

// type Output;

pub trait Add<Rhs = Self> {
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}

pub trait Sub<Rhs = Self> {
    type Output;
    fn sub(self, rhs: Rhs) -> Self::Output;
}

pub trait Mul<Rhs = Self> {
    type Output;
    fn mul(self, rhs: Rhs) -> Self::Output;
}

pub trait Div<Rhs = Self> {
    type Output;
    fn div(self, rhs: Rhs) -> Self::Output;
}

pub trait Index {
    type Output;
    fn index(self, index: u64) -> Self::Output;
}

// [a, b) &[T]
// pub trait Range<Idx> {
//     pub start: Idx,
//     pub end: Idx,
//     // fn range(self, start_inc: u64, end_exc: u64) -> Self::Output;
// }
