use std::borrow::Borrow;
use std::iter::Map;

use plotters::element::{PointCollection, PointElement};
use plotters::prelude::*;

use crate::treasure::ZoneTreasure;

pub fn plot<S: AsRef<str>>(name: &S, zone_data: &Vec<ZoneTreasure>) -> Result<(), Box<dyn std::error::Error>>{
    let name = name.as_ref();
    let x_max = zone_data.iter().map(|a| a.pos_x).max().unwrap();
    let y_max = zone_data.iter().map(|a| a.pos_y).max().unwrap();
    let x_min = zone_data.iter().map(|a| a.pos_x).min().unwrap();
    let y_min = zone_data.iter().map(|a| a.pos_y).min().unwrap();

    let x_dif = (x_max - x_min) as u32;
    let y_dif = (y_max - y_min) as u32;

    let file = format!("output/_svg/{}.svg", name);
    let root = SVGBackend::new(&file, (x_dif + 200, y_dif + 200)).into_drawing_area();
    root.fill(&WHITE)?;
    let root = root.margin(10, 10, 10, 10);
    let mut chart = ChartBuilder::on(&root)
        .caption(name, ("sans-serif", 40).into_font())
        .x_label_area_size(20)
        .y_label_area_size(40)
        .build_ranged(x_min as i32..x_max as i32, 0..y_dif as i32)?;

    chart.draw_series(PointSeries::of_element(
        zone_data, 5,&RED, &|c, s, st| {
            let pos = (c.pos_x as i32, y_max as i32 - c.pos_y as i32);
            EmptyElement::at(pos) + Circle::new((0, 0), s, st.filled())
            + Text::new(format!("{}", c.id + 1), (0, 0), ("sans-serif", 16).into_font())
        }))?;




    Ok(())

}

//
// impl<'a> PointCollection<'a, (i32, i32)> for &'a ZoneTreasure {
//     type Borrow = &'a (i32, i32);
//     type IntoIter = std::iter::Once<&'a (i32, i32)>;
//
//     fn point_iter(self) -> Self::IntoIter {
//         std::iter::once(&(self.pos_x as i32, self.pos_y as i32))
//     }
// }

