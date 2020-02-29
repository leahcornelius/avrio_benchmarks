use avrio_database::*;

fn main() {
  println!("hello world");
  saveData("hi", "./database.db", "hey");
}
