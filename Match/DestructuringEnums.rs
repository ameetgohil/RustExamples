// `allow` required to silence warnings because only
// one variant is used.
#[allow(dead_code)]
enum Color {
    // These 3 are specified solely by their name.
    Red,
    Blue,
    Green,
    // These likewise tie `u32` tuples to different names: color models.
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn main() {
    let color = Color::RGB(122, 17, 40);

    println!("What color is it?");
    // An enum can be destructured using a match
    match color {
        Color::Red => println!("The color is Red!"),
        Color::Blue => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) =>
            println!("Red: {}, Green: {}, and Blue: {}!", r, g, b),
        Color::HSV(h, s, v) =>
            println!("Hue: {}, Saturation: {}, and Value: {}!", h, s, v),
        Color::HSL(h, s, l) =>
            println!("Hue: {}, Saturation: {}, and Lightness: {}!", h, s, l),
        Color::CMY(c, m, y) =>
            println!("Cyan: {}, Magenta: {}, and Yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) =>
            println!("Cyan: {}, Magenta: {}, and Yellow: {}, key (black): {}!", c, m, y, k),
    }
}
