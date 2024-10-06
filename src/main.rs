use std::cmp::Ordering;

/// About Ord trait
/// https://doc.rust-lang.org/std/cmp/trait.Ord.html
/// Example 1: Implement an Ord for person struct
#[derive(Eq)]
struct Person {
    id: u32,
    name: String,
    height: u32,
}

/// Ord requires that the type also be PartialOrd and Eq (which requires PartialEq).

impl Ord for Person {
    fn cmp(&self, other: &Self) -> Ordering {
        self.height.cmp(&other.height)
    }
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.height == other.height
    }
}

fn bubble_sort<T>(arr: &mut [T]) where T: Ord {

  if arr.is_empty() {
    return
  }

  let mut sorted = false;
  let mut n = arr.len();

  while !sorted {
    sorted = true;

    for i in 0..n-1 {
      if arr[i] > arr[i+1] {
        arr.swap(i, i+1);
        sorted = false; 
      }
    }

    n = n - 1;
  }

}
fn main() {
    let john = Person {
        id: 1,
        name: String::from("hello"),
        height: 20,
    };

    let doe = Person {
        id: 1,
        name: String::from("hello"),
        height: 12,
    };

    if john > doe {
        println!("Yes. John is taller than doe");
    }

    let mut ve1 = vec![6, 5, 4, 3, 2, 1];
    bubble_sort(&mut ve1);
    println!("{:?}", ve1);
}
