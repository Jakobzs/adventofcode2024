struct MyStuff {
    pub x: i32,
    pub y: i32,
}

pub fn wowie() {
    let stuff = MyStuff { x: 1, y: 2 };
    println!("Wowie! {} {}", stuff.x, stuff.y);
}
