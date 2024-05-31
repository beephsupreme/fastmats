use reqwest;
use scraper::Selector;
pub const URL: &str = "https://www.toki.co.jp/purchasing/TLIHTML.files/sheet001.htm";

#[derive(Debug)]
struct ScheduleRow {
    part_number: String,
    row_len: usize,
    shipments: Vec<f32>,
}

impl ScheduleRow {
    fn new(&mut self, name: String, len: usize) -> Self {
        self.part_number = name.to_string();
        self.row_len = len;
        self.shipments = Vec::new();
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
    let mut table: Vec<Vec<String>> = Vec::new();
    for _ in 0..num_rows {
        let srow: ScheduleRow = ScheduleRow::new("testes".to_string(), row_len);
        let mut row: Vec<String> = Vec::new();
        for i in 0..row_len {
            match i {
                0 => {
                    let val1 = html[i].clone();
                    let val1 = &val1[0..val1.find("<").unwrap_or(val1.len())];
                    row.push(val1.into());
                }
                1 | 2 | 3 | 4 => continue,
                _ => {
                    let val = html[i].clone();
                    if val == "�@" {
                        row.push("".to_string());
                    } else {
                        row.push(html[i].clone());
                    }
                }
            }
        }
        table.push(row);
        html.drain(0..row_len);
    }
    table.iter().for_each(|x| println!("{:?}", x));
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
