struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn area_of_n (&self, n: u32) -> u32 {
        self.area() * n
    }

    fn degug(){
        println!("This is a rectangle struct");
    }
    
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };  
    println!("area: {}", rect.area());
    println!("area of n: {}", rect.area_of_n(3));
    Rectangle::degug(); // static method call
}
