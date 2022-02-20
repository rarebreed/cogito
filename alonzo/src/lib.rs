pub trait Functor {
    type T;

    fn fmap<B>(&self, fun: impl Fn(Self::T) -> B) -> B;
}

pub fn pipe<A, B, C>(f1: impl Fn(A) -> B, f2: impl Fn(B) -> C) -> impl Fn(A) -> C {
    move |a: A| f2(f1(a))
}

/// Takes an Iterator, and returns a list of pairs, where the first item
///
/// Example: pairs(vec![1, 2, 3]) returns [[1, 2], [2, 3], [3, 4]]
///
/// In order to make this work for any kind of iterator, we need to use generics.  Since
/// an Iterator Item is a refernce to the item and not the item itself, we also need to
/// specify a lifetime.  This is why the type declaration is a bit ugly.
pub fn pairs<'a, I, T>(it: I) -> Vec<Vec<&'a T>>
where
    I: IntoIterator<Item = &'a T>,
{
    let mut iter = it.into_iter();
    let mut first = iter.next();
    let mut storage: Vec<Vec<&'a T>> = vec![];
    loop {
        let second = iter.next();
        match (first, second) {
            (Some(f), Some(s)) => {
                storage.push(vec![f, s]);
            },
            _ => break,
        };
        first = second;
    }
    storage
}

pub struct Ranged {
    end: usize,
    state: usize,
}

impl Ranged {
    pub fn new(start: usize, end: usize) -> Self {
        Ranged { end, state: start }
    }
}

impl Iterator for Ranged {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.state <= self.end {
            let res = self.state;
            self.state += 1;
            Some(res)
        } else {
            None
        }
    }
}

pub fn range(start: usize, end: usize) -> impl Iterator<Item = usize> {
    let ranged = Ranged::new(start, end);
    ranged
}

/// Takes a collection of A1's and B1's, returning a vector of 2 element tuple.
///
/// Each element of the tuple is from the next item in the collection. If one of
/// the collections is shorter than the other, it will stop zipping
///
/// Example:
/// ```
/// let coll1 = vec![1, 2, 3, 4];
/// let coll2 = ["a", "b", "c"];
/// let zipped = zip(&coll1, &coll2) // [(1, "a"), (2, "b"), (3, "c")]
/// ```
pub fn zip<'a, 'b, A, B, A1, B1>(coll1: A, coll2: B) -> Vec<(&'a A1, &'b B1)>
where
    A: IntoIterator<Item = &'a A1>,
    B: IntoIterator<Item = &'b B1>,
{
    let mut storage: Vec<(&'a A1, &'b B1)> = vec![];
    let mut iter1 = coll1.into_iter();
    let mut iter2 = coll2.into_iter();

    loop {
        let coll1_item = iter1.next();
        let coll2_item = iter2.next();

        match (coll1_item, coll2_item) {
            (Some(f), Some(s)) => {
                storage.push((f, s));
            }
            _ => break,
        };
    }
    storage
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let arg = vec![1, 2, 3, 4];
        let paired = pairs(&arg);
        println!("{:?}", paired);

        let slice = &[1, 2, 3, 4];
        let paired2 = pairs(slice);
        assert_eq!(paired, paired2, "not equal");
    }

    #[test]
    fn test_zip() {
        let coll1 = vec![1, 2, 3, 4];
        let coll2 = ["a", "b", "c"];

        let zipped = zip(&coll1, &coll2);
        println!("{:?}", zipped);
    }
}
