fn main() {
    let mut x = 5;
    println!("The value of x is {x}");
    
    {
        let x = 8;
        println!("The value of x is inside {x}");
    }
    x += 9;
    println!("The value of x is outside {x}");
  
    let s1 = "Hello";
    let s2 = s1;
    println!("s1 is {s1}, s2 is {s2}");
}