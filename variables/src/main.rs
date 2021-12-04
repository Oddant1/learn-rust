fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x)
    }

    println!("The value of x is: {}", x);

    another_function(x, '😻');
}

fn another_function(x: i32, unit_label: char) {
    println!("Another function was given {}{}.", x, unit_label);
}
