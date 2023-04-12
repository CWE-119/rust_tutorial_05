mod enums;

fn main() {
    // loops();
    // lesson12();
    // lesson15();
    // lesson18();
    // lesson21();
    // lesson23();
    // lesson26(Color::Blue);
    // lesson28();
    // lesson29();
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

// lesson 24 and 25 (enums)
enum Direction {
    Left,
    Right,
}

pub fn which_way(){
    let go = Direction::Left;
    match go {
        Direction::Left => println!("left"),
        Direction::Right => println!("right")
    }
}

// lesson 26 (enums)
enum Color {
    Red,
    Yellow,
    Blue
}

fn lesson26(my_color: Color){
    match my_color {
        Color::Red => println!("red"),
        Color::Yellow => println!("yellow"),
        Color::Blue => println!("blue")
    }
}

// lesson27 (structs)
// this is an example

// struct ShippingBox{
//     depth:i32,
//     width: i32,
//     height:i32
// }

// let my_box = ShippingBox{
//     depth: 2,
//     width: 2,
//     height:5,
// };

// let tall = my_box.height;

// lesson28
struct GroceryItem{
    stock:i32,
    price: f64,
}

fn lesson28(){
    let cereal = GroceryItem{
        stock: 10,
        price: 2.99
    };
    println!("Stock {:?}", cereal.stock);
    println!("Stock {:?}", cereal.price);
}

// lesson 29
#[derive(Debug)]
enum Flavor{
    Sparkling, 
    Sweet,
    Fruity
}

#[derive(Debug)]
struct Drink {
    flavor: Flavor,
    fluid_oz: f64,
}

fn lesson29(){
    fn print_drink(drink: Drink){
        match drink.flavor {
            Flavor::Sparkling => println!("Sparkling"),
            Flavor::Sweet => println!("Sweet"),
            Flavor::Fruity => println!("Fruity"),
        }
        println!("oz: {:?}", drink.fluid_oz)
    }

    let sweet = Drink {
        flavor:Flavor::Sweet,
        fluid_oz:6.0
    };

    println!("{:?}",sweet);
    let fruity = Drink{
        flavor: Flavor::Fruity,
        fluid_oz: 6.9
    };

    println!("{:?}",fruity);
}

// lesson30 (tuples)

enum Access{
    Full,
}

fn one_two_three() -> (i32, i32, i32){
    (1,2,3)
}

fn lesson30(){
    let numbers = one_two_three();
    let (x, y, z) = one_two_three();
    println!("{:?}, {:?}", x, numbers.0);
    println!("{:?}, {:?}", y, numbers.1);
    println!("{:?}, {:?}", z, numbers.2);

    let (employee, access) = ("jake", Access::Full);
}

// lesson 31 (tuples)
fn lesson31(){
    let cord = (2,3);
    println!("{:?}, {:?}",cord.0, cord.1);

    // destructure 
    let (x,y) = (2,3);
    println!(" {:?} {:?}",x,y );
    let user_info = ("nahiyan", 20);
    println!("my name is: {} and my age is {}", user_info.0, user_info.1);
}

// lesson 32 (tuples)

fn coordinates() -> (i32, i32){
    (1,7)
}

fn lesson32() {
    let (x, y) = coordinates();
    if y > 5{
        println!(">5");
    }else if y < 5 {
        println!("<5");
    }else{
        println!("=5");
    }
}

// lesson 33 (Expressions)
// no code 
// lesson 34 (use of expression)
enum Access34 {
    Admin,
    Manager,
    User,
    Guest
}

fn lesson34(){
    let access_level = Access34::Guest;
    let can_access_file = match access_level{
        Access34::Admin => true,
        _ => false,
    };
}

// lesson 35 (expression practice)
fn lesson35(){
    let value = 100;
    let is_gt_100 = value > 100;
}

fn print_message35(gt_100:bool){
    match gt_100 {
        true => println!("its big"),
        false => println!("its small"),
    }
}

// lesson 36 (intermediate memory)
// no code

// lesson37 (ownership)
// no code 

// lesson 38 (practice ownership)
struct Book38 {
    pages: i32,
    rating: i32
}

fn display_page_count38(book: &Book38){
    println!("rating = {:?}", book.pages);
}

fn display_rating38(book: &Book38){
    println!("rating = {:?}", book.rating)
}

fn lesson38(){
    let book = Book38{
        pages: 58,
        rating: 9
    };
    display_page_count38(&book);
    display_rating38(&book);
}

// lesson39 (ownership) 
// write code of ownership 
