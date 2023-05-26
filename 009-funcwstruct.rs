struct Rectangle {
    width: f32,
    height: f32,
}
fn rectangle_new(width: f32, height: f32) -> Rectangle {
    Rectangle {
        width,
        height
    }
}
fn rectangle_area(_self: &Rectangle) -> f32 {
    _self.width * _self.height
}
fn main() {
    let rect = rectangle_new(10.0, 30.0);
    println!("{}", rectangle_area(&rect));
}
