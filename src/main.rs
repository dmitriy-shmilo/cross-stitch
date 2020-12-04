use std::fs;
use std::io::Write;

const TRANSPARENCY_THRESHOLD: u8 = 100;
const SIZE_FACTOR: u32 = 10;

#[allow(unused_must_use)]
fn main() {
    let in_img = image::open("in.png").unwrap().to_rgba8();
    let (width, height) = in_img.dimensions();

    let header = fs::read_to_string("header.svg").unwrap()
        .replace("{width}", &(width * SIZE_FACTOR).to_string())
        .replace("{height}", &(height * SIZE_FACTOR).to_string());

    let tile_template = fs::read_to_string("tile.svg").unwrap();

    let mut out_svg = fs::File::create("out.svg").unwrap();
    writeln!(&mut out_svg, "{}", header).unwrap();

    for x in 0..width {
        for y in 0..height {
            let pix = in_img[(x, y)];

            if pix[3] < TRANSPARENCY_THRESHOLD {
                continue;
            }

            let cx = x * SIZE_FACTOR;
            let cy = y * SIZE_FACTOR;
            let fill = format!("rgb({}, {}, {})", pix[0], pix[1], pix[2]);
            let stroke = format!("rgb({}, {}, {})", pix[0] / 4, pix[1] / 4, pix[2] / 4);

            let tile = tile_template.replace("{cx}", &cx.to_string())
                .replace("{cy}", &cy.to_string())
                .replace("{fill}", &fill)
                .replace("{stroke}", &stroke);
            writeln!(&mut out_svg, "{}", tile);
        }
    }

    writeln!(&mut out_svg, "</svg>");
}
