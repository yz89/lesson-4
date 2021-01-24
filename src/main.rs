trait Area {
    fn calc_area(&self) -> u32;
}

trait Pow2<T> {
    fn pow2(&self) -> T;
}

impl Pow2<u32> for u32 {
    fn pow2(&self) -> u32 {
        return self.pow(2)
    }
}

struct Square<T> {
    a: T,
}

impl Square<u32> {
    pub fn calc_area(&self) -> u32 {
        self.a.pow2()
    }
}

fn main() {
    let s = Square{
        a: 10,
    };
    println!("{}", s.calc_area());
}

