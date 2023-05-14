mod front_of_house;
mod back_of_house;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {
            
//         }

//         fn seat_at_table() {
            
//         }
//     }

//     mod serving {
//         fn take_order() {
            
//         }

//         fn serve_order() {
            
//         }

//         fn take_payment() {
            
//         }
//     }
// }


pub use crate::front_of_house::hosting;

pub fn eat_at_resturant() {
    // crate::front_of_house::hosting::add_to_waitlist();
    // front_of_house::hosting::add_to_waitlist();
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please.", meal.toast);
    
    hosting::add_to_waitlist();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
