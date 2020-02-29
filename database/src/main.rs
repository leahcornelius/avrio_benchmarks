use avrio_database::*;

fn main() {
  println!("hello world");
  saveData("hi".to_string(), "./database.db".to_string(), "hey".to_string());
  println!("{}", getData("./database.db".to_string(), "hey".to_string()));
}
