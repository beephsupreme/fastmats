use reqwest;
use scraper::Selector;
pub const URL: &str = "https://www.toki.co.jp/purchasing/TLIHTML.files/sheet001.htm";

fn main() {
    let mut html: Vec<String> = get_html(URL);
    html.drain(0..18);
    let index = html.iter().position(|&r| r == "TOKISTAR CODE").unwrap();
    let mut dates: Vec<String> = Vec::new();
    for i in 0..index {
        dates.push(html[i]);
    }
    for line in html {
        println!("{:?}", line);
    }
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
