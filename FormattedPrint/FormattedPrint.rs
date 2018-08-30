fn main() {

    println!("{} days ", 31);

    println!("{0}, this {1}, {1}, this is {0}", "Alice", "Bob");

    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    println!("{} of {:b} people know binary, the other doesn't", 1,2);

    println!("{number:>width$}", number=1, width=6);

    println!("{number:>0width$}", number=1, width=6);

    println!("My name is {0}, {1} {0}", "Bond", "James");

    //Create a structure which contains i32. Name it `Structure`.
    #[allow(dead_code)]
    struct Structure(i32);

    //println!("This struct `{}` won't print...", Structure(3));
    
    
}
