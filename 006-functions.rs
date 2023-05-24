use rand::Rng;
fn main() {
    let mut rng = rand::thread_rng();
    let a: i32 = rng.gen_range(0..10000);
    let b: i32 = rng.gen_range(0..10000);
    pythagoras(a, b); 
}
 
fn pythagoras(a: i32, b: i32) {
    println!("a^2 + b^2 = {}", a*a + b*b);
}
