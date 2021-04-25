use plotters::prelude::*;
use smmo_api::models::item::ItemRarity;
use std::{f32::consts::PI, io::Write, usize};
use std::{fs::File, iter};
use svg::node::element::path::Data;
use svg::Document;
// use plotters::style::BackendColor;

const ACCURACY: f32 = 100.0;

const SCALE: u32 = 100;

pub fn pie_chart(
    items: &[(u32, (u8, u8, u8), &str)],
) -> Result<String, Box<dyn std::error::Error>> {
    let total = items.iter().fold(0, |prev, (i, _, _)| i + prev);

    let size = items.len();
    // let items = items.iter().peekable();

    let document = Document::new().set("viewBox", (0, 0, 50, 100));

    let data = Data::new();

    // let mut items = items.unz;
    let size = 100 * SCALE as usize * 50 * SCALE as usize * 4;
    let mut buffer = String::with_capacity(size);
    dbg!(buffer.len());
    {
        let root_area =
            SVGBackend::with_string(&mut buffer, (100 * SCALE, 50 * SCALE)).into_drawing_area();

        let (left, right) = root_area.split_horizontally(50 * SCALE);

        let right = right.margin(6 * SCALE, 3 * SCALE, 1 * SCALE, 1 * SCALE);
        let right_half = right.split_evenly((size, 1));

        root_area.fill(&WHITE)?;
        // left.fill(&GREEN)?;
        // right_half[4].fill(&RED)?;

        let root_area = root_area.titled("Item Distribution", ("sans-serif", 4 * SCALE))?;

        // root_area.
        let radius = 20f32 * (SCALE as f32);

        let center = (25f32, 25f32);

        // let mut all_area_series = vec![];
        let mut previous_slice_end_angle = 0.0;

        for (index, (slice, colour, text)) in items.iter().enumerate() {
            let slice_angle = (*slice as f32 / total as f32) * (2.0 * PI);

            let start_angle = previous_slice_end_angle;
            let end_angle = previous_slice_end_angle + slice_angle;

            previous_slice_end_angle = end_angle;

            let arc_points = ((start_angle * ACCURACY).floor() as i32..(if index == size - 1 {
                // you gotta add the extra lil bit or else theres a gap between the first and last slices
                (((2.0 * PI) + 0.01) * ACCURACY).ceil() as i32
            } else {
                (end_angle * ACCURACY).ceil() as i32
            }))
                .map(|angle| {
                    (
                        center.0 + (radius * ((angle as f32) / ACCURACY).cos()),
                        center.1 + (radius * ((angle as f32) / ACCURACY).sin()),
                    )
                });

            let slice_area = iter::once(center)
                .chain(arc_points)
                .chain(iter::once(center));
            all_area_series.push((slice_area, colour, text));
        }

        #[allow(clippy::many_single_char_names)]
        for (index, (area, (r, g, b), text)) in all_area_series.into_iter().enumerate() {
            left.draw(&Polygon::new(
                area.clone()
                    .map(|(x, y)| (x as i32, y as i32))
                    .collect::<Vec<_>>()
                    .as_ref(),
                RGBColor(*r, *g, *b).filled(), /* .filled() */
            ))?; /*  _series(); */
            right_half[index].draw_text(
                text,
                &TextStyle::from(("sans-serif", 4 * SCALE).into_font())
                    .color(&RGBColor(*r, *g, *b)), /* (RGBColor(*r, *g, *b), 30) */
                (0, 0),
            )?;
        }
    }

    Ok(buffer)
}

fn center_of_area<DB, CT>(left: &DrawingArea<DB, CT>) -> (f32, f32)
where
    DB: DrawingBackend,
    CT: CoordTranslate,
{
    let range = left.get_pixel_range();
    let x_mid = range.0.start + (range.0.end / 2);
    let y_mid = range.1.start + (range.1.end / 2);
    (x_mid as f32, y_mid as f32)
}

#[test]
fn test_pie_chart() {
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

    File::open("test.svg").unwrap().write(chart.as_bytes());
}
