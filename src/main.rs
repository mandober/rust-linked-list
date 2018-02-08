pub mod linked;
use linked::List;

mod test;

fn main() {
  // new list
  let mut list = List::new();
  println!("{}", list);

  // append
  list.append("zero".to_string());
  println!("{}", list);

  // push some payloads
  list.push("airplane".to_string());
  list.push("broom".to_string());
  list.push("crest".to_string());
  list.push("drum".to_string());

  // Display the list
  println!("{}", list);

  list.pop_back();

  {

    let mut link = list.head.as_mut();
    while let Some(mut_node) = link {
      if let None = mut_node.next {
        break;
      }
      link = mut_node.next.as_mut();
    }

  }


  /*
  for x in list.into_iter() {
      println!("{:?}", x);
  }
  */

} //main
