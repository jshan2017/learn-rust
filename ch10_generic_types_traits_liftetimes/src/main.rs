use rand;

fn main() {
    let mut arr_i32: [i32; 30] = rand::random();
    bubble_sort(&mut arr_i32);
    println!("{:?}", arr_i32);

    let mut arr_f64: [f64; 30] = rand::random();
    bubble_sort(&mut arr_f64);
    println!("{:?}", arr_f64)
}

fn bubble_sort<T: PartialOrd + Copy>(list: &mut [T]) {
    for i in 0..list.len() {
        for j in 0..list.len() - i - 1 {
            let a = list[j];
            let b = list[j + 1];
            if a > b {
                let temp = b;
                list[j + 1] = a;
                list[j] = temp;
            }
        }
    }
}
