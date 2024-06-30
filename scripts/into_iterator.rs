// https://doc.rust-lang.org/std/iter/trait.IntoIterator.html

#[derive(Debug)]
struct MyCollection(Vec<i32>);

impl MyCollection {
    fn new() -> Self {
        MyCollection(Vec::new())
    }

    fn add(&mut self, elem: i32) {
        self.0.push(elem);
    }
}

impl IntoIterator for MyCollection {
    type Item = i32;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

fn main() {
    let mut c = MyCollection::new();
    c.add(0);
    c.add(1);
    c.add(2);
    c.add(2);

    for (i, v) in c.into_iter().enumerate() {
        println!("iter {i} val {v}");
    }
}
