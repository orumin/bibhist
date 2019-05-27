extern crate nom_bibtex;
extern crate chrono;

use chrono::prelude::*;
use nom_bibtex::*;
use svg::node;
use svg::node::element;

use std::fs;
use std::collections::HashMap;

macro_rules! yearmonth {
    ($x:expr) => {
        if $x.contains_key("month") {
            Utc.ymd($x.get("year").unwrap().parse::<i32>().unwrap(), $x.get("month").unwrap().parse::<u32>().unwrap(), 1)
        } else {
            Utc.ymd($x.get("year").unwrap().parse::<i32>().unwrap(), 1, 1)
        }
    }
}

fn main() -> Result<(), Box<std::error::Error>> {
    let contents = fs::read_to_string("history.bib")?;

    let bibtex = Bibtex::parse(&contents).unwrap();

    let mut sorted_bibliographies = bibtex.bibliographies().iter().map(|biblio| {
        biblio.tags().iter().cloned().collect::<HashMap<_, _>>()
    }).collect::<Vec<HashMap<_, _>>>();
    
    sorted_bibliographies.sort_unstable_by(|a, b| {
        let a = yearmonth!(a);
        let b = yearmonth!(b);
        a.cmp(&b)
    });

    let (document, dx) = sorted_bibliographies.iter().fold((svg::Document::new(), 0),
    |(acc, dx), biblio| {
        let date = if biblio.contains_key("month") {
            format!("data: {}/{}", biblio.get("year").unwrap(), biblio.get("month").unwrap())
        } else {
            format!("date: {}", biblio.get("year").unwrap())
        };
        (acc.add(element::Group::new()
                .add(element::Text::new()
                    .add(node::Text::new(date.to_string()))
                        .set("x", dx)
                        .set("y", 100)
                        .set("text-anchor", "start"))
                .add(element::Text::new()
                    .add(node::Text::new(format!("title: {}", biblio.get("title").unwrap())))
                        .set("x", dx)
                        .set("y", 110)
                        .set("text-anchor", "start")))

                ,dx+800)
    });

    let line = element::path::Data::new()
        .move_to((10, 50))
        .line_to((dx, 40))
        .close();
    let path = element::Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 5)
        .set("d", line);
    let document = document.add(path).set("viewBox", (0, 0, dx+50, 150));

    svg::save("image.svg", &document).unwrap();

    Ok(())
}

