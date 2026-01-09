fn add(num1: i32, num2: i32) -> i32{
    num1 + num2
}

fn main3_1(){
    println!("{}", add(1,2));
}

fn swap(num1: i32, num2: i32) -> (i32, i32){
    (num2, num1)
}

// 여러개 return
fn main3_2(){
    let (num1, num2) = swap(1,2);
    println!("{num1}, {num2}");
}

// scope
fn hello(name: String){
    let num = 3;
    println!("Hello {}", name);
}

fn main3_3(){
    let my_name = "buzzi".to_string();
    {
        println!("My name is {}", my_name);
        let my_name = "mellon";
    }

    hello(my_name);
}

// lambda
fn main3_4() {
    let my_func = |x| x+1;
    println!("{}", my_func(3));
}

// or 
fn main3_4() {
    let my_func = |x:i32| ->i32 { x + 1};
    println!("{}", my_func(3));
}

fn main3_4(){
    
}