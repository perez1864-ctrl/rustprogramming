
//Assignment 1: Temperature Converter
const FREEZING_POINT: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_POINT) * 5.0 / 9.0

}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + FREEZING_POINT
 }


 //Assignment 2:

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

// Assignment 3:
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}



fn main() {
    //Assignment 1:
    let mut temp_in_f: f64 = 32.0;

    let temp_in_c = fahrenheit_to_celsius(temp_in_f);
    println!("{}°F is {}°C", temp_in_f, temp_in_c);

    for _ in 0..5 {
        temp_in_f += 1.0;
        let temp_in_c = fahrenheit_to_celsius(temp_in_f);
        println!("{}°F is {}°C", temp_in_f, temp_in_c);
    }
    

    //Assignment 2:
    
    let numbers = [4, 5, 15, 21, 23, 6, 1, 19, 91, 93];

    for num in numbers {
        if num % 3 == 0 && num % 5 == 0 {
            println!("{}: FizzBuzz", num)
        } else if num % 3 == 0 {
            println!("{}: Fizz", num);
        } else if num % 5 == 0 {
            println!("{}: Buzz", num);
        } else if is_even(num) {
            println!("{}: Even", num);
        } else {
          println!("{}: Odd", num);
        }

    }
    

        let mut sum = 0;
        let mut i =0;

        while i < numbers.len() {
        sum += numbers[i];
        i += 1;
        }

        println!("Sum of  umbers: {}", sum);

        let mut largest = numbers[0];

        for num in numbers {
        if num > largest {
            largest = num;
        }
        }

        println!("Largest number: {}", largest);


        //Assignment 3:

        let mut secret = 6;
        let guesses = [12, 21, 4, 3, 2, 45, 6];

        let mut attempts = 0;
        let mut index = 0;

        while index < guesses.len() {
            let guess = guesses[index];
            attempts += 1;

            let result = check_guess(guess, secret);
            if result == 0 {
                println!("{} is correct!", guess);
                break;

            } else if result == 1 {
                println!("{} is too high.", guess);
            } else {
                println!("{} is too low.", guess);
            }
            index += 1;
        }

        println!("It took {} guesses.", attempts);
    
     }
