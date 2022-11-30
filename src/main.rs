use regex::Regex;
use std::env;
use std::path::PathBuf;

use pdftotext::Error;

fn main() -> Result<(), Error> {
    let path = PathBuf::from(env::args_os().nth(1).expect("File path"));

    let res: String = pdftotext::pdftotext_layout(path.to_str().unwrap())?
        .into_iter()
        .collect();

    let lines = res.lines();
    let date = Regex::new(r"^\d{2}.\d{2}.\d{4}$").unwrap();
    let name = Regex::new(r"^[A-Z][a-z]* [A-Z][a-z]*$").unwrap();
    let hours = Regex::new(r"^\d+$").unwrap();

    let iter = lines
        .map(|l| l.replace("\n\n", ""))
        .skip_while(|l| !l.contains("Summary of Worklogs"))
        .filter(|l| !l.is_empty());

    let mut names = iter.clone().filter(|l| name.is_match(l));
    let mut dates = iter.clone().filter(|l| date.is_match(l));
    let mut hours = iter.clone().filter(|l| hours.is_match(l));
    loop {
        let Some(date)= dates.next() else {
            break;
        };
        let Some(name) = names.next() else {
            break;
        };
        let Some(hours) = hours.next() else {
            break;
        };
        let (first, last) = name.split_once(' ').unwrap();
        println!(
            "{} {},{} 9:00,{} {}:00,9914,MONETA Mobile devs.,Ostatní,Otevřený,Mobile dev.",
            last,
            first,
            date,
            date,
            9 + hours.parse::<i32>().unwrap()
        );
    }
    Ok(())
}
