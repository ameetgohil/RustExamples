fn main() {
    // Increment via closures and functions
    fn function (i: i32) -> i32 { i + 1 }

    //Closures are anonymous, here we are binding them to references
    // Annotation is identiscal to function annotation but is optional
    // as are teh `{}` wrapping the body. These namess functions
    // are assigned to appropriately named variables.
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred  = |i     |          i + 1  ;

    let i = 1;
    // Call the function and closures.
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    //A closure taking no arguments which returns an i32
    // Thre return type is inferred.
    let one = || i;
    println!("closure returning one: {}", one());
}
