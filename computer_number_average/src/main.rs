use std::collections::HashMap;

fn main() {
    let mut v = vec![3, 1, 5, 2, 9, 8, 4, 10, 15, 17, 13, 12, 5, 6, 7, 8, 10, 11, 12, 15, 8];

    let sum = sum(&v);

    println!("The sum is: {}", sum);

    let average = average(&v);
    println!("The average is: {}", average);

    let median = median(&mut v);
    println!("The median is: {}", median);

    println!("the origin vector is: {:?}", v);

    let map = number_count(&v);
    println!("the occurs most often is: {:?}", map);
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

fn number_count(v: &Vec<i32>) -> (i32, i32) {
    let mut map = HashMap::new();
    for num in v {
        let count = map.entry(*num).or_insert(0);
        *count += 1;
    }
    let mut most_often_value = 0;
    let mut most_often_key = 0;
    for (key, value) in &map {
        if *value > most_often_value {
            most_often_value = *value;
            most_often_key = *key;
        }
    }

    (most_often_key, most_often_value)
}
