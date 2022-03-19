// fn main() {
//     println!("Hello, world!");
//     another_function();
//     second_function(9,8)
// }
// fn another_function(){
//     println!("Its another function");
// }
// fn second_function(x:i32,y:i32){
//     println!("The value of x & y is {} , {} respectively .",x,y);
// }
fn main() {
    let mut s1 = String::from("hello");
    s1.push_str("world");
    println!("{}", s1);

    let s2 = s1.clone();
    println!("string 2 is {}", s2);

    let s3 = s1.len();
    println!("{}", s3);
    // let s = String::from("HI I am Akash");

    // let len = calculate_length(&s);
    // println!("{} {}", s, len);



        let my_name = "AkashGarg";
        let my_country = "India";
        let my_home = "Saharanpur";
    
        let together = format!(
            "I am {} and I come from {} but I live in {}.",
            my_name, my_country, my_home
        );
    
        println!("{}",together);
    


// fn calculate_length(s: &String) -> usize {
//     s.len()

    let person1 = Person {
        name : String::from("Akash"),
        age : 23,
        height : 167.0,
    };

    println!("Person1 details {:?} {:?} {:?}", person1.age , person1.height ,person1.name);

    let rect1 = Rectangle {
        length:30,
        width:50
    };

    println!("Area of rectangle is {:?}",rect1.area());
// }


println!("Hello, world!");

// let v: Vec<i32> = Vec::new();

let mut v = vec![1, 2];

v.push(4);
v.push(5);

println!("{:?}", v);

let third = &v[2];
println!("The value of third number is {}", third);

match v.get(4) {
    Some(third) => println!("The 3rd element is {}", third),
    None => println!("There is no element exists."),
}

let v1 = vec![1, 2, 3, 4];
for i in &v1 {
    println!("{}", i);
}











}

#[derive(Debug)]
struct Person {
    name: String,
    age: u16,
    height: f32,
}

struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
}



/*Topics covered ---------------

1. String
2.struct
3. Function 
4. Vector

*/
