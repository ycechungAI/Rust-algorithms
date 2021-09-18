fn bubble_sort(items: &mut [i32]) {
    let length = items.len();

    for i in (1..length).rev() {
        for j in 0..i {
            if items[j] > items[j + 1] {
                items.swap(j, j + 1);
            }
        }
    }
}
fn main(){
    let mut array1 = [5, 9, 7, 8, 1, 2, 12, 1];
    println!("\n- Before Sort | Implementation -");
    println!("{:?}", array1);
    println!("- After Sort | Implementation -");
    bubble_sort(&mut array1);
    println!("{:?}", array1);
    println!("\n");
}
