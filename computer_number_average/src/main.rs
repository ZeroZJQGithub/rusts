fn main() {
    let mut v = vec![3, 1, 5, 2, 9, 8, 4, 10, 15, 17, 13, 12];

    let sum = sum(&v);

    println!("The sum is: {}", sum);

    let average = average(&v);
    println!("The average is: {}", average);

    let median = median(&mut v);
    println!("The median is: {}", median);

    println!("the origin vector is: {:?}", v);
}


fn sum(v: &Vec<i32>) -> f64 {
    let mut sum = 0.0;
    for num in v {
        sum += *num as f64;
    }
    sum
}

fn average(v: &Vec<i32>) -> f64 {
    let count = v.len();
    let mut sum = 0.0;
    for num in v {
        sum += *num as f64;
    }

    sum / count as f64
}

fn median(v: &mut Vec<i32>) -> f64 {
    v.sort();
    println!("the sorted vector is: {:?}", v);
    let count = v.len();
    if count % 2 == 0 {
        let sum = (v[count/2 - 1] + v[(count + 2) / 2 - 1]) as f64;
        println!("the median sum is: {}", sum);
        sum / 2 as f64
    } else {
        v[(count + 1) / 2 - 1] as f64
    }
}
