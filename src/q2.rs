
fn add_integer(ints: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;
    for i in ints {
        sum += i;
        if sum == u32::MAX {
            return None
        }
    }
    Some(sum)
}

fn main() {
    let sum = add_integer(&[1,2,3,4,5]);
    match sum {
        Some(sum) => println!("{}", sum),
        None => println!("None"),
    }

    let sum = add_integer(&[1, u32::MAX-1]);
    match sum {
        Some(sum) => println!("{}", sum),
        None => println!("None"),
    }
}