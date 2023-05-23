#[allow(unused_variables)]

use std::thread;

fn main() {
  let mut handles = vec![];

  for _num in 0..10 {
    let handle_thread = thread::spawn(|| {
      println!("Hello, {}", _num);

    });
    println!("{}", _num);
    handles.push(handle_thread);
  }

  for handle in handles {
    handle.join().unwrap();
  }

  return;
}

//Closures can access variables from their surrounding scope,
//even after the surrounding scope has finished executing.
//This is known as "capturing the environment" and allows closures
//to access variables and perform actions within that captured context.

