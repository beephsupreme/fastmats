use csv;
use scraper::Selector;
use serde::Deserialize;
use std::{collections::HashMap, error::Error};
pub const URL: &str = "https://www.toki.co.jp/purchasing/TLIHTML.files/sheet001.htm";
pub const DATA: &str = "./data/data.txt";

fn main() {
    if let Err(e) = load_data() {
        eprintln!("load_data(\"{DATA}\") : {e}");
    }

    // let (dates, schedule) = get_schedule();
    // println!("{:?}", dates);
}

#[allow(dead_code)]
fn get_html(url: &str) -> Vec<String> {
    let response = reqwest::blocking::get(url).unwrap().text().unwrap();
    // let response = fs::read_to_string("schedule.html").unwrap();
    let html = scraper::Html::parse_document(&response);
    let selector = Selector::parse("td").unwrap();
    let mut elements: Vec<String> = Vec::new();
    for element in html.select(&selector) {
        elements.push(element.inner_html());
    }
    elements
}

#[allow(dead_code)]
fn get_schedule() -> (Vec<String>, HashMap<String, Vec<f32>>) {
    let mut html: Vec<String> = get_html(URL);
    html.drain(0..18);
    let index = html.iter().position(|r| r == "TOKISTAR CODE").unwrap();
    let mut dates: Vec<String> = Vec::new();

    for i in 0..index {
        dates.push(html[i].clone());
    }

    let row_len = dates.len() + 5;
    html.drain(0..(index + row_len));
    let num_rows = (html.len() / row_len) - 1;
    let mut schedule: HashMap<String, Vec<f32>> = HashMap::new();

    for _ in 0..num_rows {
        let mut name: String = html[0].clone();
        name = (name[0..name.find('<').unwrap_or(name.len())]).to_string();
        let mut vals: Vec<f32> = Vec::new();

        for i in 5..row_len {
            let temp = html[i].clone();
            if temp == "�@" {
                vals.push(0.0);
            } else {
                match temp.parse::<f32>() {
                    Ok(t) => vals.push(t),
                    Err(e) => {
                        println!("{}/{}", temp, e)
                    }
                }
            }
        }

        match schedule.get(&name) {
            Some(v) => {
                let sum: Vec<f32> = vals.into_iter().zip(v).map(|(a, b)| a + b).collect();
                schedule.insert(name, sum);
            }
            None => {
                schedule.insert(name, vals);
            }
        }

        html.drain(0..row_len);
    }
    (dates, schedule)
}

#[allow(dead_code)]
fn translate_schedule(schedule: HashMap<String, Vec<f32>>) {
    println!();
    println!("{:?}", schedule);
}

fn load_data() -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(DATA)?;
    let headers = reader.headers()?;
    println!("{:?}", headers);

    for result in reader.deserialize() {
        let record: Data = result?;
        println!("{:?}", record);
    }
    Ok(())
}

#[derive(Debug, serde::Deserialize)]
struct Data {
    part_number: String,
    on_hand: f32,
    on_order: f32,
    reorder: f32,
    multiplier: f32,
}
