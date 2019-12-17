#[derive (Debug)]
struct Matrix (i32, i32, i32);

fn main() {
    let matrix = Matrix(1, 2, 3);
    println!("{:?}", matrix);
}
