use plotters::prelude::*;
use std::f32::consts::PI;
use std::iter;

use smmo_api::models::item::ItemRarity;

fn test(items: &[(i32, (u8, u8, u8))]) -> Result<(), Box<dyn std::error::Error>> {
    let root_area = BitMapBackend::new("sample.png", (5000, 5000)).into_drawing_area();

    root_area.fill(&WHITE)?;

    let root_area = root_area.titled("Image Title", ("sans-serif", 60))?;

    let center = {
        let range = dbg!(root_area.get_pixel_range());
        let x_mid = range.0.start + (range.0.end / 2);
        let y_mid = range.1.start + (range.1.end / 2);
        dbg!((x_mid as f32, y_mid as f32))
    };

    let items = [
        (696, ItemRarity::Uncommon),
        (3270, ItemRarity::Elite),
        (1312, ItemRarity::Common),
        (382, ItemRarity::Exotic),
        (1323, ItemRarity::Rare),
        (11136, ItemRarity::Celestial),
        (3409, ItemRarity::Epic),
        (3905, ItemRarity::Legendary),
    ];

    let total = items.iter().fold(0, |prev, (i, _)| i + prev);

    let mut all_area_series = vec![];
    let mut previous_angle_end = 0.0;
    let total_slices = items.len();
    for (iteration, (slice, rarity)) in items.iter().enumerate() {
        let frac = *slice as f32 / total as f32;
        let this_angle = frac * (2.0 * PI);
        let angle_from = previous_angle_end;
        let angle_to = previous_angle_end + this_angle;
        previous_angle_end = angle_to;
        let arc_points = ((angle_from * 100.0).floor() as i32..(if iteration == total_slices - 1 {
            // you gotta add the extra lil bit or else theres a gap between the first and last slices
            (((2.0 * PI) + 0.01) * 100.0).ceil() as i32
        } else {
            (angle_to * 100.0).ceil() as i32
        }))
            .map(|angle| {
                (
                    center.0 + (1000.0 * ((angle as f32) / 100.0).cos()/* * 180.0 / PI */),
                    center.1 + (1000.0 * ((angle as f32) / 100.0).sin()/* * 180.0 / PI */),
                )
            });
        let slice_area = iter::once(center)
            .chain(arc_points)
            .chain(iter::once(center));
        all_area_series.push((slice_area, rarity));
    }

    for (area, rarity) in all_area_series {
        #[allow(clippy::clippy::many_single_char_names)]
        let r = ((rarity.colour() >> 16) & 255) as u8;
        let g = ((rarity.colour() >> 8) & 255) as u8;
        let b = (rarity.colour() & 255) as u8;
        root_area.draw(&Polygon::new(
            area.clone()
                .map(|(x, y)| (x as i32, y as i32))
                .collect::<Vec<_>>()
                .as_ref(),
            RGBColor(r, g, b).filled(), /* .filled() */
        ))?; /*  _series(); */
    }

    Ok(())
}

#[test]
fn test_pie_chart() {
    test().unwrap();
}
