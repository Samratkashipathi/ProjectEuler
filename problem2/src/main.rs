fn main() {
    let mut values: (i64, i64) = (0, 1);
    let mut sum: i64 = 0;

    for _ in 1..40000 {
        let temp = values.1;
        values.1 = values.0 + values.1;
        values.0 = temp;

        if values.1 > 4000000 {
            break;
        }

        if values.1 % 2 == 0 {
            sum = sum + values.1;
        }
    }
    println!("{:#?}", sum);
}
