fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x)
    }

    println!("The value of x is: {}", x);

    another_function(x, '😻');

    let x = five();
    println!("The value of x is: {}", x);

    if x < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    if x != 0 {
        println!("No implicit truthiness!");
    }

    let condition = true;
    // Need to both be same type
    let x = if condition { 5 } else { 6 };

    println!("The value of x is now: {}", x);

    loop {
        println!("This loop would be infinite if we didn't immediately break");
        break;
    }

    // Nested loops
    let mut count = 0;
    // We can label loops which is neat
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    // We can return a thing from a loop by putting the value after break
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn another_function(x: i32, unit_label: char) {
    println!("Another function was given {}{}.", x, unit_label);
}

fn five() -> i32 {
    5
}
