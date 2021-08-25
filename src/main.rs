extern crate cairo;
extern crate png;
use cairo::{Context, Format, ImageSurface};
use std::fs::File;

const MODULES: i32 = 113;
const WIDTH: i32 = MODULES * 4;
const HEIGHT: i32 = MODULES * 2;

const NUMBER_L: &str = "012345";
const NUMBER_R: &str = "678905";

const BEGIN: i32 = 0b1101; // 3 bits
const QUIET: i32 = 0b1000000000;
const L: [i32; 10] = [
    0b10001101, 0b10011001, 0b10010011, 0b10111101, 0b10100011, 0b10110001, 0b10101111, 0b10111011,
    0b10110111, 0b10001011,
];
const MID: i32 = 0b101010; //5 bits
const R: [i32; 10] = [
    // 7 bits per module
    0b11110010, 0b11100110, 0b11101100, 0b11000010, 0b11011100, 0b11001110, 0b11010000, 0b11000100,
    0b11001000, 0b11110100,
];

const END: i32 = 0b1101; // 3 bits

fn draw_bar(ctx: &Context, position: i32) {
    let previous_source = ctx.source();
    const BAR_WIDTH: f64 = WIDTH as f64 / MODULES as f64;
    ctx.set_source_rgb(0.0, 0.0, 0.0);
    ctx.rectangle(position as f64 * BAR_WIDTH, 0.0, BAR_WIDTH, HEIGHT as f64);
    ctx.fill().expect("error filling rect");
    ctx.set_source(&previous_source)
        .expect("error setting source");
}

fn draw_pattern(ctx: &Context, mut pattern: i32, mut position: i32) -> i32 {
    while pattern > 1 {
        if pattern & 1 == 1 {
            draw_bar(ctx, position);
        }
        position += 1;
        pattern >>= 1;
    }
    return position;
}

fn main() {
    let surface =
        ImageSurface::create(Format::ARgb32, WIDTH, HEIGHT).expect("Couldn't create a surface!");
    let context = Context::new(&surface).expect("all good");
    context.set_source_rgb(1., 1., 1.);
    context.paint().expect("not painted");

    let mut position = 1;
    position = draw_pattern(&context, QUIET, position);

    position = draw_pattern(&context, END, position);

    for char in NUMBER_R.chars().rev() {
        position = draw_pattern(&context, R[char.to_digit(10).unwrap() as usize], position);
    }

    position = draw_pattern(&context, MID, position);

    for char in NUMBER_L.chars().rev() {
        position = draw_pattern(&context, L[char.to_digit(10).unwrap() as usize], position);
    }

    position = draw_pattern(&context, BEGIN, position);

    draw_pattern(&context, QUIET, position);

    let mut file = File::create("output.png").expect("Couldn't create file!");
    surface
        .write_to_png(&mut file)
        .expect("Couldn't write to file!");
}
