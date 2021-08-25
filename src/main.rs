extern crate cairo;
extern crate png;
use cairo::{Context, Format, ImageSurface};
use std::fs::File;

const MODULES: i32 = 113;
const WIDTH: i32 = MODULES * 4;
const HEIGHT: i32 = MODULES * 2;

const NUMBER_L: &str = "012345";
const NUMBER_R: &str = "678905";

const QUIET: i32 = 0x200;
const BEGIN: i32 = 0xd;
const MID: i32 = 0x2a;
const END: i32 = 0xd;
const L: [i32; 10] = [0x8d, 0x99, 0x93, 0xbd, 0xa3, 0xb1, 0xaf, 0xbb, 0xb7, 0x8b];
const R: [i32; 10] = [0xf2, 0xe6, 0xec, 0xc2, 0xdc, 0xce, 0xd0, 0xc4, 0xc8, 0xf4];

fn draw_bar(ctx: &Context, position: i32) {
    let previous_source = ctx.source();
    const BAR_WIDTH: f64 = WIDTH as f64 / MODULES as f64;
    ctx.set_source_rgb(0.0, 0.0, 0.0);
    ctx.rectangle(position as f64 * BAR_WIDTH, 0.0, BAR_WIDTH, HEIGHT as f64);
    ctx.fill().expect("error filling rectangle");
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
