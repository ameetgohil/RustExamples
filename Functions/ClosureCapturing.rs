fn main() {
    use std::mem;

    let color = "green";

    // A closure to print color which immediately borrows (`&`)
    // color and stores the borrow and closure in the print
    // variable. It will remain borrowed until print goes out of
    // scopoe. println! only requries by reference so it doesn't
    // impose anything more restrictive.
    let print = || println!("`color`: {}", color);

    // Call the closure using the borrow.
    print();
    print();

    let mut count = 0;
    // A closure to increment count could take eithere &mut cout
    // or count but &mut count is less restrictive so it takes
    // that. Immediately borrows count
    //
    // A `mut is required on inc because a & mut is stored inside
    // Thus, calling the closure mutates the closure which requires
    // a `mut`
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // Call the closure.
    inc();
    inc();
}
