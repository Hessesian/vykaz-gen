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
    let name = Regex::new(r"^\w+ \w+$").unwrap();

    let mut iter = lines
        .filter(|l| !l.contains("\n\n"))
        .skip_while(|l| !l.contains("Summary of Worklogs"))
        .filter(|l| l != &"")
        .filter(|l| date.is_match(l) || name.is_match(l))
        .filter(|l| !l.contains("key"));
    loop {
        let Some(date)= iter.next() else {
            break;
        };
        let Some(name) = iter.next() else {
            break;
        };
        let (first, last) = name.split_once(' ').unwrap();
        println!(
            "{} {},{} 9:00,{} 17:00,9914,MONETA Mobile devs.,Ostatní,Otevřený,Mobile dev.",
            last, first, date, date
        );
    }
    Ok(())
}
