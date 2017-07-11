fn factorial(n : u32 ) -> u32 {
    (1..n+1).fold(1, |fac, x| fac *x)
}

fn main() {
    println!("factorial 10 = {}", factorial(10));
}
