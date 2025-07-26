// fn get_str_len(s: &str)->usize{
//     s.chars().count()
// }


// fn is_even(num: i32)-> bool {
//     if num % 2 == 0{
//         return true;
//     } else {
//         return false;
//     }
// }

// fn fib(num: u32)->u32{
//     let mut first =0;
//     let mut second = 1;
//     if num==0 || num ==1{
//         return num;
//     }
//     for _ in 0..(num-1){
//         let temp = second;
//         second = second+first;
//         first = temp;
//     }
//     return  second;
// }

//struct implementation

// struct rect{
//    l: u32,
//    w: u32,
// }

// impl rect{
//     fn area(&self)->u32{
//         self.l * self.w
//     }
//     fn per(&self)->u32{
//        2*(self.l + self.w) 
//     }
//     fn debug()->u32{
//         return 1;
//     }
// }

// fn main() {
//     let rect1 = rect{
//         l:10,
//         w:10,
//     };
//     println!("area is {} and perimeter is {}",rect1.area(),rect1.per());
//     println!("debug is {}",rect::debug());
    // let ans = fib(8);
    // println!("{}", ans);
    // let my_str = String::from("Hello, world!");
    // let length = get_str_len(&my_str);
    // println!("the number of chars in the string is: {}",length);
// }

// vector implementation

// fn main(){
//     let mut vec = Vec::new();
//     // let mut vec = vec![1, 2, 3, 4, 5];
//     vec.push(1);
//     vec.push(2);
//     vec.push(3);
//     vec.push(4);
//     vec.push(5);
//     println!("the vector is {:?}", vec);
//     println!("the even numbers are {:?}", even_filter(&vec));
// }

// fn even_filter(vec: &Vec<i32>) -> Vec<i32> {
//     let mut even_vec = Vec::new();
//     for val in vec {
//         if val % 2 == 0 {
//             even_vec.push(*val);
//         }
//     }
//     return even_vec; 
// }

 //hashmap implementation

//  use std::collections::HashMap;
//  fn group_by_values(vec:Vec<(String, i32)>) -> HashMap<String,i32> {
//     let mut hm = HashMap::new();
//     for(key,value)in vec {
//         hm.insert(key, value);
//     }
//     return hm;
//  }
//  fn main() {
//     let mut users = HashMap::new();
//     users.insert(String::from("Alice"),  22);

//     let first_user_age = users.get("Alice");

//     match first_user_age {
//         Some(age) => println!("Alice's age is {}", age),
//         None => println!("User not found"),
//     }
//     let vec = vec![
//         (String::from("Bob"), 30),
//         (String::from("Charlie"), 25),
//         (String::from("David"), 35),
//     ];
//     let grouped_users = group_by_values(vec);
//     println!("Grouped users: {:?}", grouped_users);
//  }
     

     // traits implementation
    
    // trait Summary{
    //     fn summarise(&self)->String;
    // }

    // struct User{
    //     name: String,
    //     age: u32,
    // }

    // impl Summary for User {
    //     fn summarise(&self)->String {
    //         return format!("The name is {}, and the age is {}",self.name,self.age);
    //     }
    // }
    //  fn main(){
    //     let user = User{
    //         name: String::from("Ranjan"),
    //         age:22,
    //     };
    //     println!("{}",user.summarise());

    //  }

//lifetime implementation

// use std::fmt::Display;

// fn longest_with_an_announcement<'a, T>(
//     x: &'a str,
//     y: &'a str,
//     ann: T,
// ) -> &'a str
// where
//     T: Display,
// {
//     println!("Announcement! {}", ann);
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
// fn main() {
//     let str1 = String::from("Hello");
//     let str2 = String::from("World");
//     let result = longest_with_an_announcement(&str1, &str2, "Comparing lengths");
//     println!("The longest string is: {}", result);
// }

// multi-threading implementation

use std::thread;
fn main() {
    thread::spawn(||{
        for i in 1..5 {
            println!("hi from spawned Thread : {}", i);
        }
    });

    for i in 1..5 {
        println!("hi from main Thread : {}", i);
 }
}