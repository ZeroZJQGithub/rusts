fn main() {
    println!("Common Collections!");

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    
    println!("the 3rd item is: {third}");

    match v.get(2) {
        Some(third) => println!("the third item is: {}", third),
        None => println!("There is no third element."),
    }

    let mut vec_arr = vec![1, 2, 3, 4, 5, 6];
    // let mut first = &mut vec_arr[0];
    let first = vec_arr[0];
    vec_arr.push(7);
    println!("The first element is: {first}");
    println!("The vector array is: {:?}",vec_arr);
}
