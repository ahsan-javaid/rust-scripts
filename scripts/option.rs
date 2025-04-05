use std::fmt::Debug;

fn console_log<T: Debug>(v: Option<T>) {
    match v {
        Some(v) => println!("{:?}", v),
        None => println!("None")
    }
}

fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
      None 
    } else {
      Some(numerator / denominator)
    }
}

fn check_optional(optional: Option<Box<i32>>) {
    match optional {
        Some(v) => println!("value: {v}"),
        None => println!("none")
    }
}

fn add_last(stack: &mut Vec<i32>) -> Option<i32> {
    Some(stack.pop()? + stack.pop()?)
}
  

 fn main() {
    // example 1
    let result = divide(2.0, 3.0);
  
    match result {
      Some(x) => println!("Result: {x}"),
      None => println!("none")
    }
    // exmaple 2
    let optional = None;
    check_optional(optional);
    let optional = Some(Box::new(9000));
    check_optional(optional);
    //Example 3
    let mut v = vec![1,2,3];
    let r = add_last(&mut v);

    console_log(r);
}


