fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];
    let x = input.iter().min().unwrap_or(&0);
    let y = input.iter().max().unwrap_or(&0);

    println!("{} is largest and {} is smallest", y, x);
}
