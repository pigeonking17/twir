use clap::{Arg, App};
use scraper::{Html, Selector};

fn main() {
    this_week_rust("https://this-week-in-rust.org/blog/archives/index.html")
}

fn this_week_rust(url: &str) {    
    let matches = App::new("Get this week in rust web pages based on dates. Optionally gives the quote of the week. (unimplemented)")
                            .version("0.1.0")
                            .author("Luke M. <debating whether to put my email here>")
                            .arg(Arg::with_name("year")
                                .short("y")
                                .long("year")
                                .value_name("YEAR")
                                .help("4 number year to refine searches. (e.g. 2016 2021)")
                                .takes_value(true))
                            .arg(Arg::with_name("month")
                                .short("m")
                                .long("month")
                                .value_name("MONTH")
                                .help("3 letter month to refine searches. (e.g. JUL DEC)")
                                .takes_value(true))
                            .arg(Arg::with_name("day")
                                .short("d")
                                .long("day")
                                .value_name("DAY")
                                .help("2 number day to refine searches. (e.g. 13 23)")
                                .takes_value(true))
                            .arg(Arg::with_name("quote")
                                .short("q")
                                .help("Enable to get the quote of the week. (unimplemented)"))
                            .get_matches();

    let year = matches.value_of("year");
    let month = matches.value_of("month");
    let day = matches.value_of("day");
    let _quote = matches.is_present("quote");

    let mut dates_and_title = get_dates_and_titles(url);

    if year != None {
        dates_and_title = dates_and_title.into_iter().filter(|(date, _)| 
            year == date.as_str().split_whitespace().skip(2).next()).collect::<Vec<(String, String)>>();
    }
    
    if month != None {
        dates_and_title = dates_and_title.into_iter().filter(|(date, _)| 
            month == date.as_str().to_lowercase().split_whitespace().skip(1).next()).collect::<Vec<(String, String)>>();
    }

    if day != None {
        dates_and_title = dates_and_title.into_iter().filter(|(date, _)| 
            day == date.as_str().split_whitespace().next()).collect::<Vec<(String, String)>>();
    }
    
    for (date, title) in dates_and_title {
        println!("{}: {}", title, date);
    }
}

fn get_dates_and_titles(url: &str) -> Vec<(String, String)> {
    let resp = reqwest::blocking::get(url).unwrap();
    assert!(resp.status().is_success());

    let body = resp.text().unwrap();

    let fragment = Html::parse_document(&body);

    let rows = Selector::parse(".row.post-title").unwrap();

    let mut dates_and_titles = Vec::new();

    for row in fragment.select(&rows) {
        let row = Html::parse_fragment(&row.inner_html());
        let div_selector = Selector::parse("div").unwrap();
        let span_selector = Selector::parse("span").unwrap();

        let date_div = row.select(&div_selector).next().unwrap();
        let span = date_div.select(&span_selector).next().unwrap();
        let date: &str = span.text().collect::<Vec<_>>()[1];

        let a_selector = Selector::parse("a").unwrap();

        let title_div = row.select(&div_selector).skip(1).next().unwrap();
        let a = title_div.select(&a_selector).next().unwrap();
        let title: &str = a.text().collect::<Vec<_>>()[0];

        dates_and_titles.push((date.to_owned(), title.to_owned()));
    }

    dates_and_titles
}
