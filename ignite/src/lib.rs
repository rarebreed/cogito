use reqwest::{Client};

async fn post_request(json: String, token: &str) {
    let client = Client::new();
    let response = client.post("https:/my.cool.service/v2/awesome/endpoint")
        .body(json)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("Bearer ", token)
        .send()
        .await;
    match response {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => panic!("Got error: {}", e)
    }
}

pub fn concat(left: &str, right: &str) -> String {
    format!("{}{}", left, right)
}

pub fn mut_concat(left: &mut String, right: &str) {
    left.push_str(right)
}

pub fn basics(id: String) {
    let mut ids = vec![];
    ids.push(id);

    // println!("id is {}", id); // doesn;t compile
    let mut scores = vec![];
    let score = 64;
    scores.push(score);
    println!("score is {}", score);  // this works, because i32 implements Copy trait
}

trait Monoid {
    fn append(&self, rhs: Self) -> Self;
    fn empty(self) -> Self;
}

struct Foo {
    inner: String
}

struct IntWrapper {
    inner: u64
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
    use crate::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_concat() {
        let name = "sean";
        let new = concat(name, " toner");
        println!("name is {}, new name is {}", name, new);

        let mut l = "sean".into();
        mut_concat(&mut l, " toner");
        println!("left is {}", l)
    }
}
