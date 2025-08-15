// use core::slice;
// struct  Rectangle {
//      width : u32,
//      height : u32
// }
// fn main() {
//    let x = 5 ;
//    let x = x + 1 ;
//    {
//      let x = x * 2 ;
//      println!("The value of x in multiple is : {x}");
//    }
//    println!("The value of x is : {x}")
// let x= 2.0;
// println!("x value is : {x}");
// let  y = 3.0 ;
// println!("y value is : {y}");

// let tup = (500 , 0.6 , 1);
// let (x , y , z) = tup;
// println!("The value is : {y}");
// another_function(5 , 't');

// let number = 6 ;
// if number  < 5 {
// println!("Condition is true");
// }else{
//     println!("Condition is false")
// }
// let number = 4;
// if number % 4 == 0 {
//     println!("Number is devided by : 4")
// } else if number % 3 == 0 {
//    println!("Number is devide by 3")
// } else if number % 2 == 0 {
//     println!("Number is devided by 2")
// } else {
//     println!("Number is do not devided by 4 , 3 ,2 ")
// }

// loop {
//     println!("Again")
// }

// let mut count = 0 ;
// 'counting_up: loop {
//    println!("count : {count}");
//    let mut remaining = 10;
// loop {
//     println!("remaining : {remaining}");
//     if remaining == 9 {
//         break;
//     }if count == 2 {
//         break 'counting_up;
//     }
//     remaining -= 1 ;
// }
// count += 1;
// }
// println!("End count : {count}");
// let mut s = String::from("hello");
// s = String::from("manoj");
// println!("{s}, world")

// let s1 = String::from("hlo");
// let s2 = s1.clone();
// println!("s2 value is : {s2} , s1 value is : {s1}");'
//   let s1 = String::from("hello");
//   take_string(s1);
//   let x = 5 ;
//   take_interger(x);
//   println!("The value is : {x}");
    // let s1 = String::from("hello");
    // let len = take_interger(&s1);
    // println!("the value is s1 : {s1} , and the value is len is : {len}")
    // }

    // fn take_interger(s : &String) -> String{
    //     println!("{s}")
    // }
// fn take_string(some_string : String){
//       println!("{some_string}");
// }
// fn another_function(x : i32 , united : char){
//     println!("The value of x is : {:?} {united}" , x + 5);
// let arr = [10 , 20 ,30 , 40];
// let slice = &arr[1];
// println!("Slice is a :{:?} " , slice);

// let a = [1,2,3,4,5,6,7];
// let slice =  &a[1..3];
// assert_eq!(slice , &[2 ,3]);
// let  react1 = Rectangle{
//     width : 10 ,
//     height : 10
// };
// println!("This is a reactangle : {:?}" , area(&react1))



// }
// // }
// fn area(reactangle : &Rectangle)-> u32{
//       reactangle.width * reactangle.height
// }
// struct Rectangle{
//     width : u32,
//     height : u32
// }
// impl Rectangle {
//     fn area(&self)-> u32{
//        self.width * self.height
//     }
// }
// fn main(){
//     let react1 = Rectangle{
//         width : 100 ,
//         height : 20,
//     };
//     println!("This is reactangle : {:?}" , (&react1).area())
// }

// fn main(){
//     println!("This is a Manoj");
//     let v = vec![1,2,3,4,5,6,7,8,9,];
//     let third = &v[3];
//     println!("The Third elements is :{third}");
//     let third = v.get(8);
//     match third {
//         Some(third)=> println!("The third element is : {third}"),
//         None => println!("Theree is no third element"),
//     }
// }
// fn main(){
//         let  s1 = String::from("foo");
//     let s2 = String::from(" bar");
// let s3 = s1 + &s2;
// println!("The value of s3 is : {s3}");

// use std::collections::{btree_map::Values, HashMap};

// }
// fn main(){
//     let mut scores = HashMap::new();
//     scores.insert("Manoj", 60);
//     scores.insert("Vinuta", 80);
//         println!("The some score is : {:?}", scores);
//   if let Some(score) = scores.get("Manoj"){
//     println!("The Total Manoj score is : {:?}",score)

// let mut scores = HashMap::new();
// scores.insert(String::from("Monoj") , 50);
// scores.insert(String::from("Vinuta"), 80);
// println!("The Total score is : {:?}" , scores);
//  for (key , value) in &scores {
//        println!("{key} , {value}");
//  }
//   }
// }

// use core::error;
// use std::{fmt::Error, fs::File, io::ErrorKind};

// fn main(){
    // let mut s1 = String::from("The");
    // let s2 = "Manoj";
    // s1.push_str(s2);
    // println!("This is a  :{s2}")
    // panic!("crach and butn")
    // let s = vec![1,2,3,4];
    // s[99];
//     let greeting_files = File::open("hello.txt");
//     println!("{:?}",greeting_files);

//     let greeting = match greeting_files{
//         Ok(file)=> file,
//         Err(error) =>match  error.kind(){
//               ErrorKind::NotFound => match File::create("hello.txt"){
//                 Ok(fc) =>fc,
//                 Err(cf)=> panic!("Problem creating the file : {cf:?}"),
//               }
//               _=>{
//                 panic!("Error open the file ")
//               }

//         }
//     };
//     println!("{:?}",greeting);
// }

// fn largest<T:PartialOrd+ Copy>(list:&[T])-> T{
//    let mut largest = list[0];
//    for &item in list.iter() {
//        if item > largest{
//         largest = item;
//        }
//    }
// largest
// }
// fn main(){
//     let intiger = vec![1 , 2, 3, 4, 5, 6, 7 , 8 , 9];
//     let charactor = vec!['c','g','h','k'];
//     println!("The value of intiger is a : {}",largest(&intiger));
//     println!("The value of charactor is a : {}", largest(&charactor));
// }

// trait Drive {
//     fn start(&self);
// }
// struct Car;
// impl Drive for Car{
//     fn start(&self){
//    println!("Start the car")
//     }
// }
// fn main(){
//      let s1 = Car;
//      s1.start();
// }
// fn get_str<'a>(s : &'a str)-> &'a str{
    //    s
// }
// use std::rc::Rc;
// fn main(){
    // println!("This is a Manojseetaram...!");
    // let hel_str = String::from("Manoj");
    // let manoj = get_str(&hel_str);
    // println!("{}",manoj);
    //Closers is like a anonymus function you can define inline 
    // let add = vec![1 , 2 ,3 ,  4];
    // let mut iter = add.iter();
    // println!("{:?}" , iter.next())

// let a = Rc::new("Shared".to_string());
// let b = Rc::clone(&a);
// let c = Rc::clone(&a);
// let d = Rc::clone(&a);
// println!("Rc strong Count : {}", Rc::strong_count(&a));
// let b = Box::new(100);
// println!("The value is :{}" , b)
// }

// use std::ops::Deref;

// struct MyBox<T>(T);
// impl <T> Deref for MyBox<T> {
//      type Target = T;
//      fn deref(&self) -> &Self::Target{
//         &self.0
//      }
// }
// fn main(){
//     let x = MyBox("Rust" . to_string());
//     println!("Hello  {}" , *x)
// }

// use std::thread;



// fn main(){
//     println!("This is a Manojseetaram");
//     let handle = thread::spawn(||{
//         for i in 1..5 {
//             println!("This is a spawn thread : {} ", i)
//         }
//     });
//     for i in 1..3 {
//         println!("This is a main thread : {}", i)
//     }
//     handle.join().unwrap();
// }

// use std::thread;




// //single thread 
// fn main(){
//     println!("This is a Manoj");
//     let handle = thread::spawn(||{
//         for i in 1..5{
//             println!("This is a spawn thread : {}" , i)
//         }
//     });
//     for i in 1..3 {
//         println!("This is a main thread : {}" , i)
//     }
//     handle.join().unwrap();
// }


//Sending value between a Threads 
// multiple producer single consumer 

// use std::{sync::mpsc, thread};

// fn main(){
//     println!("This is a Manoj");
//     let (tx , rx ) = mpsc::channel();

//     thread::spawn(move || {
//         let a = "Hello from thread" ;
//         tx.send(a).unwrap();
//     });
//     let recived = rx.recv().unwrap();
//     println!("This is a : {}" , recived);
// }

// use std::{sync::mpsc, thread};

// fn main(){
//     println!("This is a manoj");
//     let (tx , rx) = mpsc::channel();
//     thread::spawn(move || {
//         let messege = "I love rust";
//         tx.send(messege).unwrap();

// use std::{sync::{Arc, Mutex}, thread};

//     });
//     let recived = rx.recv().unwrap();
//       println!("This i a reciver :{}" , recived);
    
// }
// fn main(){
//     let counter = Arc::new(Mutex::new(0));
//     let mut handles = vec![];

//     for _ in 0..5  {
//         let counter = Arc::clone(&counter);
//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();
//             *num += 1;

//         });
//         handles.push(handle);
//     }
//     for handle in handles{
//         handle.join().unwrap();
//     }
//     println!("final count : {}" , *counter.lock().unwrap())
// }




// fn main(){
//     let counter = Arc::new(Mutex::new(0));
//     let mut handles = vec![];

//     for _ in 0..5 {
//         let counter = Arc::clone(&counter);
//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();
//             *num += 1 ;
//         });
//         handles.push(handle);
//     };
//     for handle in handles{

// use std::{sync::Mutex, thread};

//         handle.join().unwrap();
//     }
//     println!("This is a value : {:?}" , *counter.lock().unwrap())
// }
// struct  Data;

// fn main(){
// let a = Mutex::new(Data);
// let handle = thread::spawn(move || {
//   let _unused = a.lock().unwrap();
// });
// handle.join().unwrap();
// }

// use std::{thread::sleep, time::Duration};
// use tokio::join;
// async fn boiling_water(){
//     println!("Biling water..!");
//     sleep(Duration::from_secs(3)).await;
//     println!("Water Boiled");
// }
// async fn vegetabel_water(){
//     println!("Cutting vegetabele");
//     sleep(Duration::from_secs(2)).await;
//     println!("Water");
// }
// #[tokio::main]

// use std::{thread::sleep, time::Duration};

// async fn main(){
//    let water = boiling_water();
//    let vegitabel = vegetabel_water();
//    join!(water,vegitabel);
//    println!("Time to cook")
// };
// async fn boiled_water(){
// println!("water is Boiled...!");
// sleep(Duration::from_secs(3));
// println!("Water Boiling completed");

// use tokio_stream::{self as stream, StreamExt};

// }
// async fn vegiteble(){
// println!("Vegitable cutting.");
// sleep(Duration::from_secs(2));
// println!("ccompleted veg cutting..!");
// }
// #[tokio::main]
// async fn main(){
//   vegiteble().await;
//   boiled_water().await;
//   println!("Time to cook")
// }
// async fn get_data()->i32{
//     42
// }
// #[tokio::main]
// async fn main(){
//   let num = get_data().await;
//   println!("this is a value : {:?}" , num);
// }
// #[tokio::main]
// async  fn main(){
//     println!("This is a Manojseetaram..!");
//     let mut my_stream = stream::iter(vec![1,2,3,4,5]);
//     while let Some(value)  = my_stream.next().await{
//       println!("Got : {}" , value)
//     }
// }       
// enum Status {
//     Success ,
//     Error(String)
// }
// fn print_status(status : Status){
//     match  status {
//         Status::Success => println!("All good"),
//         Status::Error(msg)=> println!("Err {}" , msg),
//     }
// }
// fn main(){
//     println!("This is a Manojseeetaram...!");
//    print_status(Status::Success);
//    print_status(Status::Error(String::from("Something went wrong")));
// }
//Pattern and matching;
// enum Order{
//     Pending,
//     Confirmed,
//     OutForDlivery,
//     Deliverd ,
//     Cancled(String)
// }
// fn handle_order_status(order : Order){
//    match order {
//        Order::Pending=>{
//         println!("Order is still pending ")
//        },
//        Order::Confirmed=>{
//         println!("Order is Confirmed");
//        },
//        Order::OutForDlivery=>{
//         println!("Order is pickup the Box");

//        },
//        Order::Deliverd=>{
//         println!("Order is Delivered");

//        },
//        Order::Cancled(reason)=> {
//         println!("order is cancled to with a reson : {}" , reason)
//        }

//    }
// }
// fn main(){
// println!("This is a manoj..!");
//    let or = Order::Pending;
//    let or1 = Order::Cancled(String::from("restorent closed"));
//    handle_order_status(or);
//    handle_order_status(or1);
// }
// unsafe fn denguruos(){
//     println!("Hey UNSAFE");
// }

// fn main(){ 
//     println!("This is a Manojseetaram..!");
//     unsafe {
//         denguruos();
//     }
// }
//Advanced Traites in Rust 

//We will implement the shape Traites and use it with a circle and Rectangle
// trait Shape {
//     fn area(&self)->f64;
// }

// struct Rectangle{
//     width : f64 ,
//     height : f64
// }

// impl Shape for Rectangle {
//     fn area(&self)-> f64{
//       self.height * self.height
//     }
// }
// fn main(){
//     println!("This is manoj");
//     // let c = Circle{radius : 6.0};
//     let b = Rectangle{width : 40.0 , height : 50.0};
//     // println!("The circle radius is : {}" , c.area());
//     println!("The rectanagle width and height total radius is :m {}" , b.area())
// }
//Advanced Traites in Rust 

// use std::fmt::Display;

// trait  Shape : Display {
//     type  Unit ;
//     fn area(&self)-> f64 ;
//     fn unit(&self)-> Self::Unit;
// }
// struct Circle {
//     radius : f64 ,
//     unit :String
// }
// impl Display for Circle {
//     fn fmt(&self , f : &mut std::fmt::Formatter<'_>)->std::fmt::Result{
//     write!(f , "Circle with Radius : {}" , self.radius)
//     }
// }
// impl Shape for Circle{
//     type  Unit =  String;
//     fn area(&self)-> f64 {
//         3.14 * self.radius * self.radius
//     }
    
//     fn unit(&self)-> Self::Unit {
//         self.unit.clone()
//     }
// }
// struct Rectangle {
//     width : f64 ,
//     height : f64 ,
//     unit : UnitType,
// }
// impl Display for Rectangle {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f ,"Reactangle with height and width  : {:?} {:?}" , self.width , self.height)
//     }
// }
// #[derive(Debug , Clone)]
// enum UnitType{
//     Meter ,
//     Inches
// }
// impl Shape for Rectangle{
//     type  Unit = UnitType;
//     fn area(&self)-> f64 {
//         self.width * self.height
//     }

//     fn unit(&self)-> Self::Unit {
//         self.unit.clone()
//     }
// }
// fn main(){
//   let c = Circle {
//     radius : 5.0 ,
//     unit : "cm".to_string()
//   };
//   let b = Rectangle {
//     width : 40.0 ,
//     height : 50.0 ,
//     unit : UnitType::Meter
//   };
//   println!("This is a Circle Total Radius : {} : {} {:?}" , c  , c.area() , c.unit() );
//   println!("This is a Rectangle {} : {}  {:?}" , b   , b.area() , b.unit());



// }

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:8081").expect("Port bind failed");

    println!("Server running at http://127.0.0.1:8081");

    for stream in listener.incoming() {
        let stream = stream.expect("Failed to accept connection");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    let _ = stream.read(&mut buffer).unwrap_or(0);

    let request = String::from_utf8_lossy(&buffer[..]);

    let (status_line, content) = if request.starts_with("GET / ") {
        (
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n",
            "<h1>Welcome to my Rust Server</h1>",
        )
    } else if request.starts_with("GET /hello/") {
        (
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n",
            "<h1>Hello From Rust</h1>",
        )
    } else {
        (
            "HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/html\r\n\r\n",
            "<h1>Page Not Found</h1>",
        )
    };

    let response = format!("{}{}", status_line, content);

    let _ = stream.write_all(response.as_bytes());
    let _ = stream.flush();
}
