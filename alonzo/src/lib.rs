//! Crate for functional style data structures, typeclasses and functions

#![deny(missing_docs)]

/// A "typeclass" for a Functor
///
/// TODO: When Generic Associated Types are in beta, use a GAT
pub trait Functor {
    /// Represents the type that fmap takes
    type T;

    /// fmap is how a Functor type can transform data, or map from one range (output) to another
    fn fmap<B>(&self, fun: impl Fn(Self::T) -> B) -> B;
}

/// A more intuitive form or compose, where order is from left to right
///
/// Example:
///
/// ```rust
/// # use alonzo::pipe;
/// let double = |x| { x * 2};
/// let plus10 = |x| { x + 10};
/// let piped = pipe(double, plus10);
/// let answer = piped(3);  // should equal 16
/// assert!(answer == 16);
/// ```
pub fn pipe<A, B, C>(f1: impl Fn(A) -> B, f2: impl Fn(B) -> C) -> impl Fn(A) -> C {
    move |a: A| f2(f1(a))
}

/// Takes an Iterator, and returns a list of pairs, where the first item
///
/// Example:
///
/// ```rust
/// # use alonzo::pairs;
/// let arg = vec![1, 2, 3];
/// let paired = pairs(&arg);
/// println!("{:?}", paired); // returns [[1, 2], [2, 3], [3, 4]]
/// ```
///
/// In order to make this work for any kind of iterator, we need to use generics.  Since
/// an Iterator Item is a refernce to the item and not the item itself, we also need to
/// specify a lifetime.  This is why the type declaration is a bit ugly.
pub fn pairs<'a, I, T>(it: I) -> Vec<Vec<&'a T>>
    where I: IntoIterator<Item = &'a T>
{
    let mut iter = it.into_iter();
    let mut first = iter.next();
    let mut storage: Vec<Vec<&'a T>> = vec![];
    loop {
        let second = iter.next();
        match (first, second) {
            (Some(f), Some(s)) => {
                storage.push(vec![f, s]);
            }
            _ => break,
        };
        first = second;
    }
    storage
}

/// An Iterator that can be used to represent a range
pub struct Ranged {
    end: usize,
    state: usize,
}

impl Ranged {
    /// Creates a new Ranged object
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

/// Like rust's .. but in function form
pub fn range(start: usize, end: usize) -> impl Iterator<Item = usize> {
    Ranged::new(start, end)
}

/// Takes a collection of A1's and B1's, returning a vector of 2 element tuple.
///
/// Each element of the tuple is from the next item in the collection. If one of
/// the collections is shorter than the other, it will stop zipping
///
/// Example:
///
/// ```rust
/// # use alonzo::zip;
/// let coll1 = vec![1, 2, 3, 4];
/// let coll2 = ["a", "b", "c"];
/// let zipped = zip(&coll1, &coll2);
/// println!("zipped is {:?}", zipped); // [(1, "a"), (2, "b"), (3, "c")]
/// ```
pub fn zip<'a, 'b, A, B, A1, B1>(coll1: A, coll2: B) -> Vec<(&'a A1, &'b B1)>
    where A: IntoIterator<Item = &'a A1>,
          B: IntoIterator<Item = &'b B1>
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

/// concatenates two str by creating a new String
pub fn concat(left: &str, right: &str) -> String {
    format!("{}{}", left, right)
}

/// concatenates a mut String with a str returning the modified mut String
pub fn mut_concat(left: &mut String, right: &str) {
    left.push_str(right)
}

trait Monoid {
    fn append(&self, rhs: Self) -> Self;
    fn empty(self) -> Self;
}

struct Foo {
    inner: String,
}

struct IntWrapper {
    inner: u64,
}

impl Monoid for IntWrapper {
    fn append(&self, rhs: Self) -> Self {
        IntWrapper { inner: self.inner + rhs.inner }
    }

    fn empty(self) -> Self {
        self
    }
}

impl Monoid for Foo {
    fn append(&self, rhs: Self) -> Self {
        Foo { inner: format!("{}{}", self.inner, rhs.inner) }
    }

    fn empty(self) -> Self {
        self
    }
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
