struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn other(&self, other: &Rectangle) {
        if self == other {
            println!("Yep, that's me!");
        } else {
            println!(
                "Nope, that guy's dimensions and area are: x {}, y {}, a {}", 
                other.width, other.height, other.area()
            );            
        }
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size, height: size
        }
    }
}
impl PartialEq for Rectangle {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height
    }
}

fn main() {
    let rect1 = Rectangle{
        width: 30, height: 50
    };
    let rect2 = Rectangle{
        width: 10, height: 40
    };
    let rect3 = Rectangle{
        width: 60, height: 45
    };
    let rect4 = Rectangle::square(20);

    println!("Area of rect1 via function: {}", area(&rect1));
    println!("Area of rect1 via method: {}", rect1.area());
    rect1.other(&rect1);
    rect1.other(&rect2);
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can rect1 hold rect4? {}", rect1.can_hold(&rect4));
    println!("Can rect2 hold rect4? {}", rect2.can_hold(&rect4));
    println!("Can rect3 hold rect4? {}", rect3.can_hold(&rect4));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}