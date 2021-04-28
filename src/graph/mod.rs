use smmo_api::models::item::ItemRarity;
use std::{f32::consts::PI, io::Write, usize};
use svg::node::{
    self,
    element::{path::Data, Path, Text},
};
use svg::Document;
use usvg::fontdb;
// use plotters::style::BackendColor;

const ACCURACY: f32 = 100.0;

const SCALE: u32 = 100;

pub fn pie_chart(
    items: &[(u32, (u8, u8, u8), &str)],
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let total = items.iter().fold(0, |prev, (i, _, _)| i + prev);

    let item_text_height = 50 / items.len();
    // let items = items.iter().peekable();

    let mut document = Document::new().set("viewBox", (0, 0, 100, 50));

    let radius = 20f32;

    let center = (25f32, 25f32);

    let mut previous_slice_end_angle = 0.0;

    for (index, (slice, (r, g, b), text)) in items.iter().enumerate() {
        let path = Path::new();

        let slice_angle = (*slice as f32 / total as f32) * (2.0 * PI);

        let start_angle = previous_slice_end_angle;
        let end_angle = previous_slice_end_angle + slice_angle;

        previous_slice_end_angle = end_angle;

        // move to arc start point
        // arc to arc end point
        // line to center
        // enclose
        let data = Data::new()
            .move_to((
                // x
                center.0 + (radius * start_angle.cos()),
                // y
                center.1 + (radius * start_angle.sin()),
            ))
            .elliptical_arc_to((
                radius,                                // rx
                radius,                                // ry
                0,                                     // x-axis-rotation
                0, // large-arc-flag TODO: check if slice is over 50%
                1, // sweep-flag
                center.0 + (radius * end_angle.cos()), // x
                center.1 + (radius * end_angle.sin()), // y
            ))
            .line_to((
                center.0, // x
                center.1, // y
            ));

        let colour = format!("rgb({},{},{})", r, g, b);
        let path = Path::new().set("d", data).set("fill", &*colour);

        let text = Text::new()
            .set("x", 50)
            .set("y", (item_text_height * index) + (item_text_height))
            .set("fill", colour)
            .set("font-family", "sans serif")
            .set("font-size", item_text_height)
            // .set("textLength", 50)
            // .set("lengthAdjust", "spacingAndGlyphs")
            .add(node::Text::new(*text));

        document = document.add(path).add(text);
    }

    let s = document.to_string();
    let mut fontdb = usvg::fontdb::Database::new();
    let font_data = include_bytes!("../../data/Roboto-Bold.ttf").to_vec();
    fontdb.load_font_data(font_data);
    let tree = usvg::Tree::from_str(
        &s,
        &usvg::Options {
            // fontdb,
            ..usvg::Options::default()
        },
    )?;

    let size = tree.svg_node().size.to_screen_size();
    let mut pixmap =
        tiny_skia::Pixmap::new(size.width() * 10, size.height() * 10).ok_or("pad pixmap")?;
    dbg!(&pixmap);
    println!("{}", s);
    let img = resvg::render(
        &tree,
        usvg::FitTo::Height(size.height() * 10),
        pixmap.as_mut(),
    )
    .ok_or("bad render")?;
    // img.save_png(outfile)?;
    // dbg!(data);

    Ok(pixmap.encode_png()?)
}

#[test]
fn test_pie_chart() {
    env_logger::init();
    let items = [
        (
            696u32,
            ItemRarity::Uncommon.colour_rgb(),
            &*ItemRarity::Uncommon.to_string(),
        ),
        (
            3270,
            ItemRarity::Elite.colour_rgb(),
            &*ItemRarity::Elite.to_string(),
        ),
        (
            1312,
            ItemRarity::Common.colour_rgb(),
            &*ItemRarity::Common.to_string(),
        ),
        (
            382,
            ItemRarity::Exotic.colour_rgb(),
            &*ItemRarity::Exotic.to_string(),
        ),
        (
            1323,
            ItemRarity::Rare.colour_rgb(),
            &*ItemRarity::Rare.to_string(),
        ),
        (
            11136,
            ItemRarity::Celestial.colour_rgb(),
            &*ItemRarity::Celestial.to_string(),
        ),
        (
            3409,
            ItemRarity::Epic.colour_rgb(),
            &*ItemRarity::Epic.to_string(),
        ),
        (
            3905,
            ItemRarity::Legendary.colour_rgb(),
            &*ItemRarity::Legendary.to_string(),
        ),
    ];
    let chart = pie_chart(&items).unwrap();

    std::fs::File::create("test.png")
        .unwrap()
        .write_all(&chart)
        .unwrap();
}

#[test]
fn test_asdf() {
    std::env::set_var("RUST_LOG", "fontdb=trace");
    env_logger::init();

    let mut db = fontdb::Database::new();
    let now = std::time::Instant::now();
    db.load_system_fonts();
    db.set_serif_family("Times New Roman");
    db.set_sans_serif_family("Arial");
    db.set_cursive_family("Comic Sans MS");
    db.set_fantasy_family("Impact");
    db.set_monospace_family("Courier New");
    println!(
        "Loaded {} font faces in {}ms.",
        db.len(),
        now.elapsed().as_millis()
    );

    const FAMILY_NAME: &str = "Times New Roman";
    let query = fontdb::Query {
        families: &[fontdb::Family::Name(FAMILY_NAME), fontdb::Family::SansSerif],
        weight: fontdb::Weight::BOLD,
        ..fontdb::Query::default()
    };

    let now = std::time::Instant::now();
    match db.query(&query) {
        Some(id) => {
            let (src, index) = db.face_source(id).unwrap();
            if let fontdb::Source::File(ref path) = &*src {
                println!(
                    "Font '{}':{} found in {}ms.",
                    path.display(),
                    index,
                    now.elapsed().as_micros() as f64 / 1000.0
                );
            }
        }
        None => {
            println!("Error: '{}' not found.", FAMILY_NAME);
        }
    }
}
