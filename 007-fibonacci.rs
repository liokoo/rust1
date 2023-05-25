fn main(){
    let n=10; 
    let res = fibonacci(n); 
    println!("{res}");
}
 
fn fibonacci(n: u64) -> u64 {
    if n <= 1 {
        return 1;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}
