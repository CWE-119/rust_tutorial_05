fn main() {
    // loops();
    // lesson12();
    // lesson15();
    // lesson18();
    // lesson21();
    lesson23()
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

// lesson 16 (match)
fn lesson16(){
    let some_bool = true;
    match some_bool {
        true => println!("its ture"),
        false => println!("false"),
    }

    let some_int = 3;
    match some_int {
        1 => println!("its 1"),
        2 => println!("its 2"),
        3 => println!("its 3"),
        _ => println!("its something else")
    }
}

// lesson 17 (making decision with match)
fn lesson17(){
    let my_name = "Bob";
    match my_name {
        "jacob" => println!("that's my name"),
        "Bob" => println!("not my name"),
        "Alice" => println!("Hello Alice"),
        _ => println!("nice to meet you")
    }
}

// lesson 18 (decision making with match)
fn lesson18(){
    let bools = true;
    match bools {
        true => println!("its true"),
        false => println!("its false"),
    }
}

// lesson 19 (decision making with match)
fn lesson19(){
    let nums = 1;
    match nums {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("no number")
    }
}

// lesson 20 (repetition with loop)
fn lesson20(){
    let mut i = 3;
    loop{
        println!("{:?}", i);
        i = i - 1;
        if i == 0{
            break;
        }
    }
    println!("done!");
}

// lesson 21 (repetition with loop)
fn lesson21(){
    let mut i = 0;
    loop {
        i = i + 1;
        if i == 4 {
            break;
        }
        println!("{:?}", &i);
    }
    println!("{:?}", &i);
    println!("done!")
}

// lesson 22 (repetition with while)
fn lesson22(){
    let mut i = 1;
    while i <= 3 {
        println!("{:?}", i);
        i = i+1;
    }
}

// lesson 23 (repetition with while)
fn lesson23(){
    let mut i = 5;
    while i >= 1 {
        println!("{:?}", i);
        i = i - 1;
    }
    println!("done!");
}