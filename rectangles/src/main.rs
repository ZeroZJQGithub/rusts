#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }
}

fn main() {
    println!("Using struct data to computer rectangle's area!");

    let weight = 20;
    let height = 30;

    let area = area(weight, height);
    println!("the rectangle's area is {area}");

    let rect1 = (30, 20);
    let area1 = area_truple(rect1);
    println!("the rectangle's area is {area1}");

    let rect2 = Rectangle {
        width: 20,
        height: 30
    };
    println!("the rectangle is {:?}", rect2);
    let area2 = area_struct(&rect2);
    println!("the rectangle's area is {area2}");

    let rect3 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("the rectangle's area is {}", rect3.area());
    println!("Can rect3 hold rect2? {}", rect3.can_hold(&rect2));

    let rect4 = Rectangle::square(20);
    println!("the rectangle4 is {:?}", rect4);

}


fn area(weight: u32, height: u32) -> u32 {
    weight * height
}

fn area_truple(diementions: (u32, u32)) -> u32 {
    diementions.0 * diementions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
