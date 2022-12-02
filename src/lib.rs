use std::{io::Read, path::PathBuf};

use regex::Regex;

pub enum InputType<'a> {
    MEM(&'a [u8]),
    FROM(Box<dyn Read>),
    PATH(PathBuf),
}

pub fn parse_text(data: InputType) -> Result<String, String> {
    let lines = match data {
        InputType::MEM(p) => pdf_extract::extract_text_mem(p).unwrap(),
        InputType::FROM(p) => pdf_extract::extract_text_from(p).unwrap(),
        InputType::PATH(p) => pdf_extract::extract_text(p).unwrap(),
    };

    let date = Regex::new(r"^\d{2}.\d{2}.\d{4}$").unwrap();
    let name = Regex::new(r"^[A-Z][a-z]* [A-Z][a-z]*$").unwrap();
    let hours = Regex::new(r"^\d+$").unwrap();

    let iter = lines
        .lines()
        .map(|l| l.replace("\n\n", ""))
        .skip_while(|l| !l.contains("Summary of Worklogs"))
        .filter(|l| !l.is_empty());
    let mut names = iter.clone().filter(|l| {
        let l = diacritics::remove_diacritics(l);
        name.is_match(&l)
    });
    let mut dates = iter.clone().filter(|l| date.is_match(l));
    let mut hours = iter.clone().filter(|l| hours.is_match(l));
    let mut output = String::new();
    let Some(name) = names.next() else {panic!("No name");};
    loop {
        let Some(date)= dates.next() else {
            break;
        };
        let Some(hours) = hours.next() else {
            break;
        };
        let (first, last) = name.split_once(' ').unwrap();
        output += &format!(
            "{} {},{} 9:00,{} {}:00,9914,MONETA Mobile devs.,Ostatní,Otevřený,Mobile dev.\n",
            last,
            first,
            date,
            date,
            9 + hours.parse::<i32>().unwrap()
        );
    }
    Ok(output)
}
