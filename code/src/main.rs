use rayon::prelude::*;

use std::collections::HashMap;
use std::env;
use std::error::Error;

mod clean;
mod read;

fn main() -> Result<(), Box<dyn Error>> {
    let file_name = env::args().nth(1).ok_or("Usage: cargo run <filename>")?;
    let chunks = read::ChunkedLines::new(file_name, 10000)?;
    // Use `par_bridge` for turning the iterator into a parallel iterator
    // Then process each chunk with `proc_chunks` and merge the results
    let records = chunks.par_bridge()
        .filter_map(Result::ok)
        .map(clean::proc_chunks)
        .reduce_with(|acc, chunk| {
            // Merge two HashMaps
            let mut acc = acc;
            for (key, (min, max, sum, count)) in chunk {
                let entry = acc.entry(key).or_insert((min, max, 0, 0));
                entry.0 = i16::min(entry.0, min);
                entry.1 = i16::max(entry.1, max);
                entry.2 += sum;
                entry.3 += count;
            }
            acc
        })
        .unwrap_or_default();
    let format_records = records
        .par_iter()
        .map(| (site, (min, max, sum, count))|
            (
                site.to_string(),
                format!(
                    "{:.1}/{:.1}/{:.1}",
                    *min as f32 / 10.0, (*sum / (*count * 10)) as f32, *max as f32 / 10.0,
                )
            )
        )
        .collect::<HashMap<String, String>>();
    print!("{:#?}", format_records);
    Ok(())
}
