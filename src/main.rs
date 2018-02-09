pub mod linked;
use linked::List;

mod test;

fn main() {
  // new list
  let mut list = List::new();
  //println!("new: {}", list);

  // append
  // list.append("zero".to_string());
  // println!("after append: {}", list);

  // push some payloads
  list.push("airplane".to_string());
  list.push("broom".to_string());
  list.push("crest".to_string());
  list.push("drum".to_string());

  // Display the list
  println!("list after 4 pushes: {}", list);

  //println!("peek_last: {:?}", list.peek_last());


  let mut it = list.iter_mut();
  while let Some(element) = it.next() {
    println!("iter_mut.next: {}", element);
  }


  /*
  for x in list.into_iter() {
      println!("{:?}", x);
  }
  */



} //main
