
#[derive(Debug)]
struct Rectangle {
    height : i32,
    width : i32
}

impl Rectangle {
    fn square(s : i32) -> Rectangle {
     Rectangle{
         height: s,
         width : s
     }
    }

    fn area(&self) -> i32{
        self.height * self.width
    }

    fn can_hold(&self, other : &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let r1 = Rectangle {
        height : dbg!(5 * 6),
        width : 7
    };

    let r2 = Rectangle {
        height : 23,
        width : 6
    };
    let r3 = Rectangle {
        height : 21,
        width : 8
    };

    println!("Area of {:?} is {}", r1, r1.area());

    let square = Rectangle::square(4);
    println!("Area of square {:?} is {}", square, square.area());

    println!("R1 can hold r2 {}", r1.can_hold(&r2));
    println!("R1 can hold r3 {}", &r1.can_hold(&r3));
}
