fn main() {
    let numbers: Vec<i32> = (1..1000).collect();

    let sum: i32 = numbers.iter().filter(|x| *x % 3 == 0 || *x % 5 == 0).sum();

    println!("{:#?}", sum);
}