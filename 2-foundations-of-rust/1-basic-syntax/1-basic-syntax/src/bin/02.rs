fn main() {
    if bigger(10, 20) != 0 {
        println!("10 is bigger than 20");
    } else {
        println!("10 still isn't bigger than 20");
    }
}

fn bigger(a: i32, b: i32) -> i32 {
    if a > b {
        return 1;
    } else {
        return 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_biggers() {
        assert!(bigger(20, 10));
        assert!(!bigger(10, 20));
    }
}
