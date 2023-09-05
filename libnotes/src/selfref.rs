use anyhow::{Error, Result};
use std::{
    fs::read_to_string,
    thread::{sleep, spawn},
    time::Duration,
};

pub fn read() -> Result<()> {
    let input = read_to_string("libnotes/data/input.txt")?;

    let handle = spawn(move || {
        let good_lines = input
            .lines()
            .filter(|lines| {
                matches!(lines
                    .chars()
                    .next(),
                    Some(c) if c.is_uppercase()
                )
            })
            .collect::<Vec<_>>();

        good_lines.iter().for_each(|line| {
            sleep(Duration::from_millis(500));
            println!("finished processing: {line}");
        });
    });
    match handle.join() {
        Ok(_) => Ok(()),
        Err(e) => {
            let e = format!("{:#?}", e);
            let err = Error::msg(e).context("failed to join threads.");
            Err(err)
        }
    }
}
