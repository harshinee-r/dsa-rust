pub fn fibon(n: &mut i32) {
    let mut n = *n;
    let mut x = 0;
    let mut y = 1;
    let mut z = x + y;
    let c = n;
    if n == 1 {
        println!("the {}th fibonocii number is {}", c, x);
    } else if n == 2 {
        println!("the {}th fibonocii number is {}", c, y);
    } else {
        n = n - 2;
        while n != 0 {
            z = x + y;
            x = y;
            y = z;
            n = n - 1;
        }
    }
    println!("the {}th fibonocii number is {}", c, z);
}
