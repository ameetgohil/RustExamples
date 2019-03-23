fn apply<F>(f: F) where
    F: Fn() {
    f();
}

fn main() {
    let x = 7;

    // Captures `x` into an anonymous type implement
    // `Fn` for it. Store it in `print`
    let print = || println!("{}", x);

    apply(print);
}
