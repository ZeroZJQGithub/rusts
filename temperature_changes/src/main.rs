use std::io;

fn main() {
    println!("Temperature Changes!");

    loop {
        println!("F is F->C; C is C->F; Q is quit. Input your selection:");
        let mut degree_type = String::new();
        io::stdin().read_line(&mut degree_type).expect("Failed to read line");
        let degree_type = degree_type.trim();

        if degree_type == "Q" {
            println!("Quit!");
            break;
        } else if degree_type != "F" && degree_type != "C" {
            println!("Please input F or C!");
        }

        println!("Input convert degree:");
        let mut convert_degree = String::new();
        io::stdin().read_line(&mut convert_degree).expect("Failed to read line");
        let convert_degree: f32 = convert_degree.trim().parse().expect("Please input number!");

        if degree_type == "F" {
            let celsius = (convert_degree - 32.0f32)/1.8f32;
            println!("Fahrenheit degree {convert_degree} corresponds to Celsius is {celsius}");
        } else {
            let fahrenheit = 32.0f32 + convert_degree * 1.8f32;
            println!("Celsius degree {convert_degree} corresponds to Fahrenheit is {fahrenheit}");
        }
    }
    

}
