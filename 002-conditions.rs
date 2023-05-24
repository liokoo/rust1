fn main(){
    let n = 30;
    if n % 3 == 0 && n % 10 == 0 {
        println!("AB");
    } else if n % 3 == 0 {
        println!("A");
    } else if n % 5 == 0 {
        println!("B");
    } else {
        println!("{}", n);
    }
}
