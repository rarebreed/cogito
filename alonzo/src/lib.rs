

trait Functor {
    type T;

    fn fmap<B>(&self, fun: impl Fn(Self::T) -> B) -> B;
}

fn pipe<A, B, C>(f1: impl Fn(A) -> B, f2: impl Fn(B) -> C) -> impl Fn(A) -> C {
    move |a: A| {
        f2(f1(a))
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
