// Author(s): Michael Koeppl

use std::io;

// Takes the numbers slice given to it and looks at its middle number. If the
// number we're looking for is smaller, we use everything left of it
// as the next slice to search in. If it's bigger, we look at everything right
// of it.
// If the number equals the one in the middle, we found our number.
fn search_num(num: i32, numbers: &[i32], start_pos: usize) -> usize {
    let middle_pos = numbers.len() / 2;

    if num < numbers[middle_pos] {
        println!("{} is smaller than {}", num, numbers[middle_pos]);
        return search_num(num, &numbers[0 .. middle_pos], start_pos)
    } else if num > numbers[middle_pos] {
        println!("{} is bigger than {}", num, numbers[middle_pos]);
        return search_num(num, &numbers[middle_pos .. numbers.len()], start_pos + middle_pos)
    } else {
        return start_pos + middle_pos;
    }
}

fn print_numbers(numbers: &[i32]) {
    print!("Available numbers: ");
    for number in numbers {
        print!("{} ", number);
    }
    println!("\n");
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    print_numbers(&numbers);   

    // Read input from user.
    println!("Which number are you looking for?");
    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).expect("Failed to read from stdin");
    match input_text.trim().parse::<i32>() {
        Ok(i) => {
            println!("{} found at position {} (counting from 0)", i, search_num(i, &numbers, 0));
        },
        Err(..) => {
            println!("No valid input")
        }
    }
}
