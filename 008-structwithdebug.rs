#[derive(Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

fn main() {
    let color1 = Color {
        r: 200,
        g: 10,
        b: 30,
    };
    let color2 = Color {
        g: 100,
        r: color1.r,
        b: color1.b,
    };
    let color3 = Color { b: 100, ..color1 };
    println!("{:?} {:?} {:?}", color1, color2, color3);
}
