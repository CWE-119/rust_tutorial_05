use std::vec;
use std::collections::HashMap;

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
    // lesson41();
    // lesson42();
    // lesson44();
    // lesson46();
    // lesson52();
    // lesson54();
    // lesson55();
    // lesson60();
    // lesson62();
    // lesson64();
    // lesson65();
    // lesson68()
    // lesson72();
    // lesson80();
    // lesson81();

    // lessen85()

    //  lesson 87 and 88
    lessen87()
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
struct GroceryItem39{
    quantity: i32,
    id:i32
}

fn display_quantity39(item: &GroceryItem39){
    println!("quantity: {:?}",item.quantity);
}

fn display_id39(item: &GroceryItem39){
    println!("id: {:?}", item.id);
}

fn lesson39(){
    let my_item = GroceryItem39{
        quantity:3,
        id: 99
    };
    display_quantity39(&my_item);
    display_id39(&my_item);
}

// lesson40 (impl)
struct Temperature40{
    degrees_f:f64,
}

impl Temperature40{

    fn freezing40() -> Self { // here the Self means the Temperature (name of the impl)
        Self { degrees_f: 32.0 }
    }

    fn boiling40() -> Self{
        Self { degrees_f: 212.0 }
    }

    fn show_temp40(&self){  // it means we have the Temperature (Temperature40 )somewhere in the program and we are referring to that
        println!("{:?} degree F", self.degrees_f);
    }
}

fn lesson40(){
    let hot = Temperature40 {degrees_f: 99.9};
    hot.show_temp40();

    let cold = Temperature40::freezing40();
    cold.show_temp40();

    let boiling = Temperature40::boiling40();
    boiling.show_temp40();
}

// lesson41 (practice impl)

struct Dimension41{
    width: f64,
    height: f64,
    depth: f64
}

impl Dimension41{
    fn print(&self){
        println!("width {:?}", self.width);
        println!("height {:?}", self.height);
        println!("depth {:?}", self.depth);
    }
}

enum BoxColor21 {
    Red,
    Brown
}

impl BoxColor21 {
    fn print(&self){
        match self {
            BoxColor21::Brown => println!("brown"),
            BoxColor21::Red => println!("red")
        }
    }
}

struct ShippingBox41{
    dimension : Dimension41,
    weight: f64,
    color: BoxColor21
}

impl ShippingBox41{
    fn new(weight:f64, color:BoxColor21, dimension: Dimension41) -> Self{
        Self{
            weight,
            color,
            dimension
        }
    }

    fn print(&self){
        self.color.print();
        self.dimension.print();
        println!("weight {:?}", self.weight);
    }
}

fn lesson41(){
    let small_dimension = Dimension41{
        width:1.0,
        height: 2.0,
        depth: 3.0
    };
    let small_box = ShippingBox41::new(5.0, BoxColor21::Red, small_dimension);
    small_box.print();
}

// lesson42 (vectors)
// multiple data and must be same type, can add , remove and traverse the entries

fn lesson42(){
    let my_number = vec![1,2,3];

    let mut my_number = Vec::new();
    my_number.push(1);
    my_number.push(2);
    my_number.push(3);
    my_number.pop();
    my_number.len();

    let two = my_number[1];
    println!("{}", two);

    for num in my_number{
        println!("{:?}", num);
    }
}


// lesson 43 (use of vector)
struct Test43{
    score:i32,
}

fn lesson43(){
    let my_score = vec![
        Test43 {score:90},
        Test43 {score:99},
        Test43 {score:77},
        Test43 {score:93},
    ];

    for test in my_score{
        println!("score = {:?}:", test.score);
    }
}

// lesson 44 (practice of vector)
fn lesson44(){
    let my_numbers = vec![10,20,30, 40];
    for num in &my_numbers{
        match num {
            30 => println!("thirty"),
            _ => println!("{:?}",num ),
        }
    }
    println!("number if element {:?}", my_numbers.len() );
}


//lesson 45 (string) 
fn lesson45(){
    let string_slice = "this is a string slice";
    let pure_string = String::from("this is a pure string");
}

// lesson46 (usage of string)
struct LineItem46{
    name: String,
    count: i32,
}

fn print_name46(name: &str){
    println!("name: {:?}", name);
}

fn lesson46(){
    let receipt = vec![
        LineItem46{
            name:"cereal".to_owned(),
            count:1},
            LineItem46{
                name:String::from("fruit"),
                count:1}
            ];

    for item in receipt{
        // println!("{:?}", item.name);
        print_name46(&item.name);  // here name is a string , in the function we assigned the function to String data type and it cannot print a borrowed string data type so, we need to change the data type that's why we used & sign to change the String to string slice 
        println!("count {:?}", item.count)
    }
}

// lesson 47 (application with string and string slices)
struct PersonInfo47 {
    name: String,
    age: i32,
    fav_color: String
}

fn print47(data: &str){
    println!("{:?}", data)
}

fn lesson47(){

    let persons = vec![
        PersonInfo47{
            name: String::from("Samiul"),
            age: 17, 
            fav_color: String::from("Aquamarine")
        },
        PersonInfo47{
            name: String::from("Nahiyan"),
            age: 20, 
            fav_color: String::from("Cyan")
        },
        PersonInfo47{
            name: String::from("Khan"),
            age: 25, 
            fav_color: String::from("Purple")
        }
    ];

    for people in persons {
        if people.age <= 10 {
            print47( &people.name);
            print47( &people.fav_color);
        }else {
            println!("fuck you");
        }
    }

}


// lesson 48 (derive)

#[derive(Debug,Clone, Copy)]
enum Positoin48 {
    Manager,
    Supervisor, 
    Worker
}
#[derive(Debug, Clone, Copy)]
struct Employee48 {
    position : Positoin48, 
    work_hour: i64,
}

fn print_employee48(emp: Employee48){
    println!("{:?}", emp)
}

fn lesson48() {

    let me = Employee48{
        position: Positoin48::Worker,
        work_hour: 40
    };
    // for derive debug print we can automatically print the function 
    print_employee48(me);  // this is a clone and a copy 
}

// lesson 50 (enum revisited)

enum Mouse50{
    LeftClick, 
    RightClick, 
    MiddleClick,
    Scroll(i32), 
    Move(i32, i32)
}

enum PromoDiscount50 {
    NewUser, 
    Holiday(String)
}

enum Discount50 {
    Percent(f64),
    Flat(i32), 
    Promo(PromoDiscount50),
    Custom(String)
}

// lesson51 (advance Match)
enum Discount51 {
    Percent(f64),
    Flat(i32), 
    Promo(PromoDiscount50),
    Custom(String)
}

struct Ticket51{
    event: String, 
    price: i32
}

fn lesson51(){

    let n = 3;
    match n {
        3 => println!("three"),
        other  => println!("number: {:?}", other),
    }

    let flat = Discount51::Flat(2);
    match flat {
        Discount51::Flat(2) => println!("flat 2"),
        Discount51::Flat(amount) => println!("flat discount of {:?}", amount),
        _ => (), // blank bracket means returning nothing 
    }

    let concert = Ticket51{
        event: "concert".to_owned(),
        price: 50,
    };
    match concert {
        Ticket51 { price:50, event } => println!("event @ 50 {:?}", event),
        Ticket51 { price, .. } => println!("price = {:?}", price),
    }
}

// lesson 52 (advance match practice)

enum Ticket52 {
    Standard(f64),
    Backstage(String, f64),
    Vip(String, f64),
}


fn lesson52(){
    let tickets = vec![
        Ticket52::Backstage("billy".to_owned(), 50.0),
        Ticket52::Standard(15.0), 
        Ticket52::Vip("Amy".to_owned(), 30.0),
    ];
    for ticket in tickets{
        match ticket {
            Ticket52::Backstage(holder,price ) => println!("standard ticket holder name {:?} and price {:?}", holder, price),
            Ticket52::Standard(price) => println!("standhard ticket price {:?}", price),
            Ticket52::Vip(holder,price) => println!("Vip ticket holder name {:?} and price {:?}", holder, price), 
            _ => println!("i don't know what you are looking for ")
        }
    }
}


// lesson 53 (option type)

// enum Option<T> {
//     Some(T),  //  it represents some Data
//     None,  // it represents no data
// }

struct Customer53 {
    age: Option<i32>,
    email: String
}

fn lesson53(){
    let mark = Customer53 {
        age: Some(22), email: "mark@gmail.com".to_owned(),
    };
    let beaky = Customer53 {
        age: None, email: "beacky@gmail.com".to_owned(),
    };

    match beaky.age {
        Some(age) => println!("customer is {:?} years old", age),
        None => println!("customer age not provided")
    }
}

struct GroceryItem53 {
    name: String,
    qty: i32
}

fn find_quantity53(name: &str) -> Option<i32>{
    let groceries = vec![

    GroceryItem53 {name : "banana".to_owned(), qty:4},
    GroceryItem53 {name : "eggs".to_owned(), qty:12},
    GroceryItem53 {name : "bread".to_owned(), qty:1},
    ];
    for item in groceries{
        if item.name == name{  // if item name is equal to the name we provided
            return Some(item.qty);  // it will return the quantity of that item 
        }
    }
    None  // if no item with that name it will return None 
}

// lesson 54 (option can be used)
struct Survey54 {
    q1: Option<i32>,
    q2: Option<bool>,
    q3: Option<String>
}

fn lesson54(){
    let response = Survey54{
        q1: Some(12),
        q2: Some(true),
        q3: Some("A".to_owned())
    };

    // checking the q1 of response
    match response.q1 {
        Some(ans) => println!("{:?}", ans),
        None => println!("q1 no response")
    }
    // checking the q2 of response
    match response.q2 {
        Some(ans) => println!("{:?}", ans),
        None => println!("q1 no response")
    }
    // checking the q3 of response 
    match response.q3 {
        Some(ans) => println!("{:?}", ans),
        None => println!("q1 no response")
    }
}

// lesson 55 ( optional data utilization )
struct Student55 {
    name: String,
    locker: Option<i32>
}

fn lesson55(){
    let mary = Student55{
        name: "Mary".to_owned(),
        locker: Some(3)
    };
    println!("student {:?}", mary.name);
    match mary.locker {
        Some(num) => println!("locker number {:?}", num),
        None => println!("no locker assigned")
    }
}

// lesson 56 Documentation 
// learn it ur-self not important now 


// lesson57 
// boring stuff

// lesson 59 (Result)
// successful Ok(T)
// error(E)

// enum Result<T, E>{
//     Ok(T),
//     Err(E)
// }


fn lesson59() {
    // this is just an example code , it will not work cuz there is no type as SoundData
    // fn get_sound(name: &str ) -> Result<SoundDate, String>{
    //     if name == "alert"{
    //         Ok(SoundDate::new("alert")),
    //     }else {
    //         Err("unable to find sound data".to_owned())
    //     }
    // }
    
    // let sound = get_sound("alert");
    // match sound {
    //     Ok(_) => println!("located sound data"),
    //     Err(e) => println!("error {:?}", e)
    // }
}

// lesson 60 (use of Result<T, E>)

#[derive(Debug)]
enum MenuChoice60{
    MainMenu,
    Start,
    Quit
}

fn get_choice60(input: &str) -> Result<MenuChoice60, String>{
    match input{
        "mainmenu" => Ok(MenuChoice60::MainMenu),
        "start" => Ok(MenuChoice60::Start),
        "quit" => Ok(MenuChoice60::Quit),
        _ => Err("menu chioce not found".to_owned())
    }
}

// normal way
fn print_choice60(choice: &MenuChoice60){
    println!("choice = {:?}", choice);
}

// second way
fn pick_choice60(input: &str) -> Result<(), String>{ // () means return  none 
    let choice:MenuChoice60 = get_choice60(input)?;
    print_choice60(&choice);
    Ok(())
}

fn lesson60(){

    // first way
    let choice: Result<MenuChoice60, _> = get_choice60("mainmenu");
    match choice {
        Ok(inner_choice) => print_choice60(&inner_choice),
        Err(e) => println!("error = {:?}", e)
    }
    // second way
    pick_choice60("start");

}


// lesson 61 (practice of Result)
struct Customer61{
    age:i32
}

fn try_purchanse61(customer: &Customer61) -> Result<(), String>{  // first type is the OK tyye and 2nd type is Err type , if ok nth returns and if error String will return 
    if customer.age < 21 {
        Err("customer must be 21 years old".to_owned()) // if not ok it will return string
    }else{
        Ok(())  // if ok no data
    }
}

fn lesson61(){
    let ashly = Customer61{age:61};
    let purched = try_purchanse61(&ashly);
    println!("{:?}", &purched);
}

// lesson 62 Results and the question mark operator 
enum Position62{
    Maintenance,
    Marketing,
    Manager,
    LineSupervisor,
    KitchenStaff,
    AssemblyTech
}

enum Status62 {
    Active, 
    Terminated
}

struct Employee62{
    position: Position62,
    status: Status62
}

fn try_access62(employee62: &Employee62) -> Result<(), String>{
    match employee62.status {
        Status62::Terminated => return Err("terminated".to_owned()), // if the status is Terminated the code will end here because of the early return
        _ => (),
    }
    match employee62.position {
        Position62::Maintenance => Ok(()),
        Position62::Marketing => Ok(()),
        Position62::Manager => Ok(()),
        _ => Err("invalid position".to_owned())
    }
}

fn print_access62(employee62: &Employee62) -> Result<(), String>{
    let attempt_access = try_access62(employee62)?;  // ? return us the OK or Err of the result type of the function, normally a function with the result type return , returns us Result type, but if we want only OK or Err we can use ? operator (you can see lesson 59)
    println!("access ok");
    Ok(())

}

fn lesson62(){
    let manager = Employee62{
        position: Position62::KitchenStaff,
        status: Status62::Active 
    };
    match print_access62(&manager) {
        Err(e) => println!("access denied {:?}", e),
        _ => ()
    }
}

// lesson 63 (hashmap)

// hashmap does not keep the order of the items 
// hashmap can have same data type values
// .insert()  to insert value 
// .remove()  to remove value 
// for (key, value) in name.iter(){}

// lesson 64 (write code in hashmap)
#[derive(Debug)]
struct Locker64{
    content: String,
}
fn lesson64(){
    let mut locker64 = HashMap::new();
    locker64.insert(1, Locker64{content:"Locker 1".to_owned()});
    locker64.insert(2, Locker64{content:"Locker 2".to_owned()});
    locker64.insert(3, Locker64{content:"Locker 3".to_owned()});

    for (k, v) in locker64.iter(){
        println!("key {:?} and the value {:?}", k, v);
    }
}

// lesson 65 (program that utilizes hashmap)

fn lesson65(){
    let mut stock = HashMap::new();
    stock.insert("chair", 5);
    stock.insert("bed", 3);
    stock.insert("table", 2);
    stock.insert("couch", 0);

    let mut total_stock = 0;
    for (k, v) in stock.iter(){
        total_stock = total_stock + v;
        let stock_count = if v == &0{
            "out of stock ".to_owned()
        }
        else{
            format!("{:?}", v)
        };
        println!("item ={:?}, stock = {:?}", k, v)
    }
// not finished yot
}


// lesson 66 (closers)
fn lesson66(){
    let add = |a,b| a + b;
    add(5, 6);
}

// lesson 67 (map combinator)

// while working with data when you transform things from one to another is called mapping 
// map works with option and it need a function to work with 

// fn maybe_num67() -> Option<i32> {

// }
// fn maybe_word67() -> Option<i32> {

// }
// fn lesson67(){
//     let plus_one = match maybe_num67(){
//         Some(num) => Some(num + 1),
//         None => None
//     };
//     let plus_one = maybe_num67().map(|num|num + 1);

//     let word_length = maybe_word67().map(|word| word.len());
//     // not finished 
// }

// lesson 68 (practice map combinator )
#[derive(Debug)]
struct User68{
    user_id:i32,
    name: String
}

    // locates a user id based on the name 
            // it takes a borrowed string
fn find_user68(name: &str) -> Option<i32>{  // here it returns an option<i32> which is an fucntion
    let name = name.to_ascii_lowercase();
    match name.as_str() {
        "sam" => Some(1),  // if the name matches it returns the some(numbers) which is an i32
        "matt" => Some(2),
        "katie" => Some(3),
        _ => None
    }
}

fn lesson68(){
    let user_name = "sam";
    let user = find_user68(user_name) // and option functions a option function we can use use a map function which checks the number of the name 
    .map(|user_id| // inside the pipe |parameter| which here is i32
        User68{  // here is the body which makes a struct
        user_id, // here it put the parameter or name number of the name 
        name: user_name.to_owned()  // here it puts the name and set it to String cuz struct takes string 
    });

    // print out the user struct if found or a "not found " if not 
    match user {  // here it checks the user's return
        // we are using some because map returns iter and iter returns some
        Some(user) => println!("{:?}", user),  // if return is Some(user), it prints the usee
        None => println!("user not found")
    }
}

// lesson 69 (Option combinator)
fn lesson69(){
    let a: Option<i32> = Some(1);
    dbg!(a);
    let a_is_some = a.is_some(); // returns True or False if have Data
    dbg!(a_is_some);
    let a_is_none = a.is_none(); // returns True or False if have no Data
    dbg!(a_is_none);
    let a_mapped = a.map(|num|num + 1);
    dbg!(a_mapped);
    let a_filter= a.filter(|num| num == &1); // if value true we keep else we throw (when we use filter we borrows the number)
    dbg!(a_filter);
    let a_or_else = a.or_else(|| Some(5));  // if have data return nothing else if have no data returns Some(5)
    dbg!(a_or_else);
    let unwrapped = a.unwrap_or_else(|| 0); // makes the Option<i32> to i32 and assign the variable 
    dbg!(unwrapped);
}
// lesson 70 (option combinator)
enum Access70{
    Admin,
    User,
    Guest
}

fn maybe_access(name: &str) -> Option<Access70>{
    match name {
        "admin" => Some(Access70::Admin),
        "gary" => Some(Access70::User),
        _ => None
    }
}

fn root70() -> Option<Access70>{
    Some(Access70::Admin)
}


fn check_part71() -> bool{
    maybe_access("admin")  // first it check if it has the access or not
    .is_some() // if yes than it returns True or it will return False
}

fn check_part72() -> Option<Access70>{
    // "root" is equivalent to Access::Admin, but it is 
    // not listed in the maybe_access function
    //  Note: Use or_else and root().
    maybe_access("root").or_else(||root70())
}

fn check_part73() -> Access70{
    maybe_access("Alice").unwrap_or_else(||Access70::Guest)
}

fn lesson70(){
    check_part71();
    check_part72();
    check_part73();
}

// lesson 71 (iterator)

fn lesson71(){
    let number = vec![1,2,3,4,5];

    let plus_one: Vec<_> = number.iter().map(|num| num + 1 ).collect();
    // let plus_one1: Vec<_> = number.iter().filter(|num| num <= 1 ).collect();
    // let plus_one2: Vec<_> = number.iter().find(|num| num == 1 ).collect();
}

// lesson72 (utilize iterator)
fn lesson72(){
    let data = vec![1,2,3,4,5];

    let iter_data: Vec<_> = data.iter().map(|num| num * 3).filter(|num| num > &10).collect();
    for i in iter_data{
        println!("{:?}", i)
    }
}

// lesson 73 (ranges)

fn lesson73(){
    let range = 1..=3; // here it will include the 3
    let range = 1..3;  // here it will not include 3

    for i in 1..4{
        println!("{:?}", i)
    }

    for i in 'a'..'f'{
        println!("{:?}", i)
    }
}


// lesson 74 (if let )
enum Color74{
    Red,
    Blue
}
fn lesson74(){



    let maybe_user = Some("jerry");
    match maybe_user {
        Some(user) => println!("user = {:?}", user),
        None => println!("No user")
    }


    // if let matches the one specific thing i am looking for 

    if let Some(user) = maybe_user{
        println!("user = {:?}", user)
    }else{
        println!("its not red")
    }

    let red = Color74::Red;
    if let Color74::Red = red{
        println!("its red");
    }else {
        println!("its not red");
    }
}


// lesson 75  while let 
fn lesson75(){
    let mut data = Some(3);

    while let Some(i) = data{
        println!("loop");
        data = None;
    }

    let number = vec![1,2,3];
    let mut number_iter = number.iter();
    while let Some(num) = number_iter.next() {  // .next will return a optional value if there is a value to return 
        println!("num = {:?}", num);
    }
    println!("done");
}

// lesson 76

pub mod greet76{
    
    pub fn hello76(){
        println!("hello")
    }
    
    pub fn goodbye76(){
        println!("goodbye")
    }
    
}

pub mod maths76{

    pub fn add76(a:i32, b:i32) -> i32{
        a+b
    }
    
    fn sub76(a:i32, b:i32) -> i32{
        a - b
    }

}


fn lesson76(){
    use greet76::*;  // first we have to import  the module into the scope to use and ::* means import everything from that module
    greet76::hello76();

    use maths76::*;
    maths76::add76(1,1);
}

// lesson77 (inline modules)


// lesson 78 (testing)
// need to write code

// lesson 79 (Testing)
// need to code

// lessen80 (external crate)
use humantime::format_duration;
use libp2p::futures::sink::Buffer;
use std::time::Duration;
fn lesson80(){
    let d = Duration::from_secs(9876);
    println!("{}",format_duration(d));
}

// lesson 81
use chrono::prelude::*;

fn lesson81(){
    let utc: DateTime<Utc> = Utc::now();
    println!("{}", utc);
    let local: DateTime<Local> = Local::now();
    println!("{}", local);
    println!("{}",local.format("%Y-%m-%d %H:%M:%S").to_string());
}

//lesson82 external Modules
// do it later
// lesson 83 external Modules
// do it later

// lesson 84 (user input)
use std::io;

fn get_input84() -> io::Result<String>{
    let mut buffer = String::new();
    io::stdin()  // this is standard in that read data form the terminal
    .read_line(&mut buffer)?; // it will read line if ok it will give us the text
    Ok(buffer
        .trim() // enter is also counted as data so we need to trim it
        .to_owned() // trim returns a borrowed string, we turned it to String
    )
}

fn lessen84(){
    let mut all_input = vec![];
    let mut times_input = 0;
    while times_input < 2 {
        match get_input84() {
            Ok(words) => {
                all_input
                .push(words);
                times_input +=1;
            }
            Err(e) => println!("{e}")
        }
    }

    for input in all_input{
        println!("Original: {:?}, capitalized: {:?}", input, input.to_uppercase());
    }
}


// lesson 85 (user input)
enum PowerState85 {
    
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate

}

// use a match expression to convert the user input into the power state enum
impl PowerState85 {
    fn new(state: &str) -> Option<PowerState85>{
        let state = state
        .trim()
        .to_lowercase();

        // String -> &str 
        match state.as_str() {
            "off" => Some(PowerState85::Off),
            "sleep" =>Some(PowerState85::Sleep),
            "reboot"=> Some(PowerState85::Reboot),
            "shutdown" => Some(PowerState85::Shutdown),
            "hibernate" => Some(PowerState85::Hibernate),
            _ => None
        }
    }
}

fn print_power_action85(state: PowerState85){
    use PowerState85::*;
    match state{
        Off => println!("turning off"),
        Sleep => println!("Sleeping"),
        Reboot => println!("rebooting"),
        Shutdown => println!("shutting down"),
        Hibernate => println!("hibernating")
    }
}

fn lessen85(){
    let mut buffer = String::new();
    let user_input = io::stdin().read_line(&mut buffer);
    if user_input.is_ok(){
        match PowerState85::new(&buffer)  // this returns Options so we need to match Some or none
        {
            Some(state)=>{print_power_action85(state)}
            None => println!("invalid power state")
        }
    }
    else
    {
        println!("Error Reading input");
    }
}

// lesson 86 
// interactive billing Application
// nth here

// lesson 87 , 88 , 89 , 90 (project)

#[derive(Debug, Clone)]
struct Bill{
    name: String,
    amount: f64 
}

struct Bills{
    inner : Vec<Bill>
}

impl Bills{
    fn new() -> Self{
        Self { inner: vec![] }
    }

    fn add(&mut self, bill: Bill){
        self.inner.push(bill);
    }

    fn get_all(&self) -> Vec<&Bill>{
        self.inner
        .iter()  // it automatically borrow
        .collect()  // it collect the iterator in to the new Vec
    }
}

fn get_input87() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err(){
        println!("print enter your data again")
    }
    let input = buffer.trim();
    if &input == &""{
        None
    }else{
        Some(input.to_string())
    }
}

enum MainMenu87{
    addBill,
    viewBill,
}


impl MainMenu87{
    fn from_str(input: &str) -> Option<MainMenu87>{
         match input {
            "1" => Some(MainMenu87::addBill),  // both are same shit 
            "2" => Some(Self::viewBill),
            _ => None
         }
    }

    fn show(){
        println!("");
        println!(" == Bill Manager ==");
        println!("1. Add Bill");
        println!("2. View Bill");
        println!("");
        println!("Enter selection");
    }
}

fn lessen87(){
    // create bill structure
    loop {
        MainMenu87::show();
        // display the menu 
        let input = get_input87()
        .expect("no data entered");

        match MainMenu87::from_str(input.as_str()){
            Some(MainMenu87::addBill) => (),
            Some(MainMenu87::viewBill) => (),
            None => return, // return to exit out of the program 
        }

        // need to continue from 90
        
        // make a choice, based on input
    }
}