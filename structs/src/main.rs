// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

// fn main() {
//     let user1 = User {
//         email: String::from("another@example.com"),
//         username: String::from("anotherusername567"),
//         active: true,
//         sign_in_count: 1,
//     };
// }

// #[derive(Debug)]
// struct Rectangle {
//     length: u32,
//     width: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         length: 50,
//         width: 30,
//     };

//     println!("rect1 is {:#?}", rect1);
// }

// #[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.length * self.width
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         length: 50,
//         width: 30,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         rect1.area()
//     );
// }

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

fn main() {
    let rect1 = Rectangle {
        length: 50,
        width: 30,
    };
    let rect2 = Rectangle {
        length: 40,
        width: 10,
    };
    let rect3 = Rectangle {
        length: 45,
        width: 60,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
