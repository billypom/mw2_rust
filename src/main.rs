fn main () {
// simple print
    println!("hello world");

// print with number or string
    let x = 10;

    println!("the number is: {}", x);
// print with other data type

    let xs = vec![3, 4, 5];
    println!("the vector is: {:?}", xs);
// if you want your own data type to be printable for debugging and logging -
// in most cases, you can add #[derive(Debug)] above their definition
// i tried...
    // #[derive(Debug)]
    // struct my_struct(i32);
    // let xy = my_struct(2);
    // println!("my struct is: {}", xy);

// Printing Errors
    println!("this is information");
    eprintln!("this is an error!!!!");

//     A note on printing performance
// Printing to the terminal is surprisingly slow! If you call things like println! in a loop, it can easily become a bottleneck in an otherwise fast program. To speed this up, there are two things you can do.

// First, you might want to reduce the number of writes that actually “flush” to the terminal. println! tells the system to flush to the terminal every time, because it is common to print each new line. If you don’t need that, you can wrap your stdout handle in a BufWriter which by default buffers up to 8 kB. (You can still call .flush() on this BufWriter when you want to print immediately.)
    



}