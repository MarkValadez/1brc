use std::{error::Error, collections::HashMap};


pub fn proc_chunks(
    chunk: Vec<String>,
) -> HashMap<String, (i16, i16, i32, i32)>
{
    let mut records: HashMap<String, (i16, i16, i32, i32)> = HashMap::new();
    let parse_line = |records: &mut HashMap<String, (i16, i16, i32, i32)>, line: &str| {
        _parse_line(records, Ok(line.to_string()))
    };
    chunk.iter()
        .try_for_each(|line| parse_line(&mut records, line))
        .unwrap();
    records
}

fn _parse_line(
    records: &mut std::collections::HashMap<String, (i16, i16, i32, i32)>,
    line: Result<String, std::io::Error>,
) -> Result<(), Box<dyn Error>> {
    match line {
        Err(e) => Err(e.into()),
        Ok(line) => {
            let (site, temp) = line.rsplit_once(';').ok_or("Incorrect line format")?;
            let int_temp = parse_temp(temp)?;
            add_entry(records, site, int_temp);
            Ok(())
        }
    }
}

fn add_entry(
    records: &mut std::collections::HashMap<String, (i16, i16, i32, i32)>,
    site: &str,
    temp: i16,
) {
    let entry = records.entry(site.to_string()).or_default();
    entry.0 = i16::min(entry.0, temp);
    entry.1 = i16::max(entry.1, temp);
    entry.2 += temp as i32;
    entry.3 += 1;
}

fn parse_temp(temp: &str) -> Result<i16, std::num::ParseIntError> {
    // String is of the form -99.9 to 99.9. We remove decimal to return i16.
    let mut scaled_temp = String::with_capacity(temp.len() - 1);
    scaled_temp.push_str(&temp[..(temp.len() - 2)]);
    scaled_temp.push(temp.chars().last().unwrap());
    scaled_temp.parse::<i16>()
}