use std::fs::File;

use std::io::{Write,Read};


 const ENEMY_LIST_PATH: &str = "assets/enemy_list.csv";
const PATH : &str= "src/main_game/enemy/enemy_list.rs";  
pub fn main(){
  let mut writefile = std::fs::OpenOptions::new().write(true).open(PATH);
  if writefile.is_err() {
    writefile = std::fs::File::create(PATH);
  } 
  
  let mut file = std::fs::File::open(ENEMY_LIST_PATH ).expect("Undefind file");
  let mut buf = String::new();
  let result = file.read_to_string(&mut buf);
  if result.is_err() {
    panic!("Unload file");
  }
  
  writefile.unwrap().write_fmt(format_args!("pub const ENEMY_LIST_STR : &str = \"{}\";", buf)).unwrap();
  println!("update_enemy_list");

}
