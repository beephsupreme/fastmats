use reqwest;
use scraper::Selector;
pub const URL: &str = "https://www.toki.co.jp/purchasing/TLIHTML.files/sheet001.htm";

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct ScheduleRow {
    part_number: String,
    shipments: Vec<f32>,
}

impl ScheduleRow {
    fn new(name: String, quantities: Vec<f32>) -> ScheduleRow {
        ScheduleRow {
            part_number: name.to_string(),
            shipments: quantities,
        }
    }
}

fn main() {
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
    let mut table: Vec<ScheduleRow> = Vec::new();
    for _ in 0..num_rows {
        let mut row: ScheduleRow = ScheduleRow::new(String::new(), Vec::new());
        for i in 0..row_len {
            let temp = html[i].clone();
            match i {
                0 => {
                    row.part_number = (&temp[0..temp.find("<").unwrap_or(temp.len())]).to_string();
                }
                1 | 2 | 3 | 4 => continue,
                _ => {
                    if temp == "�@" {
                        row.shipments.push(0.0);
                    } else {
                        row.shipments.push(temp.parse().unwrap());
                    }
                }
            }
        }
        table.push(row);
        html.drain(0..row_len);
    }

    let mut schedule: Vec<ScheduleRow> = Vec::new();
    let temp = table[0].clone();
    schedule.push(temp);
    table.drain(0..1);

    table.iter().for_each(|x| println!("{:?}", x));
    println!("{:?}", schedule);
}

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
