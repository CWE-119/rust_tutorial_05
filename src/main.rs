use std::collections::btree_map::Values;

fn main() {
    // loops();
    lesson12();
    lesson15();
}

fn add(a:i64, b:i64) -> i64{
    a + b
}


// video6

// video 7
// - loop
// - while

fn loops(){
    let mut a = 0;
    loop{
        if a == 5{
            break;
        }
        println!("{:?}", a);
        a = a+1;
        println!("{:?}", a);
    }
}


fn whiles(){
    let mut a = 0;
    while a != 5 {
        // println!("{:?}", &a);
        a = a + 1;
        println!("{:?}", &a);
    }
}

// video 11
fn lesson11(){
    let sum = 2 +2;
    let Value = 10 - 5;
    let division = 10 / 2;
    let mult = 5 * 5;

    fn sub(a:i32,b:i32) -> i32{
        a - b
    }
    println!("{}",sub(10, 5) );

    // reminder
    let rem = 6 % 3;
    let rem2 = 6 % 4;
}

// lesson 12
fn lesson12() {
    fn sum(a:i32, b:i32) -> i32{
        a + b
    }

    fn display_result(result:i32){
        println!("{}", result );
    }
    let result = sum(5, 6);
    display_result(result)
}

// lesson 13
fn lesson13(){
    let age = 15;
    if age >= 21{
        println!("ok to purchase");
    }else{
        println!("cannot purchase");
    }
}

// lesson 14
fn lesson14(){

    let a = true;
    fn display_message(a:bool){
        if a == true {
            println!("hello");
        }else{
            println!("goodbye");
        }
    }
}

// lesson 15
fn lesson15(){
    let n = 7;
    if n > 5 {
        println!(">5");
    }else if n < 5{
        println!("<5")
    }else{
        print!("=5")
    }
}