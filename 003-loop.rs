fn main(){
  let mut counter = 0;
  let value = loop {
  counter += 1;      
  if counter >= 100 {
      break counter * counter;
      }
  };
  println!("{value}");
}
