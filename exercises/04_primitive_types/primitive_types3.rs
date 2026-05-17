fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    let a = [10; 100]; // the syntax [x; y] indicates an array of `y` elements, all initialized to `x`

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }

    let mut sum = 0;
    for n in a {
        sum += n;
    }
    println!("Sum: {sum}");
}
