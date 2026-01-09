fn main1() {
    println!("Hello, world!");
}

fn main2(){
    let x: f64 = 1.2;
    let y = x as i32;
    println!("x = {}, y = {}", x, y);

}

fn main3(){
    let mut x = 5;
    x =2;
    println!("x = {}", x);
}

const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn main4(){
    println!("{}", THRESHOLD);
    println!("is_big(5) = {}", is_big(5));
}

fn main(){
    let mut x = 1;
    let y = 2;
    x +=y ;
    println!("x = {}", x);
}