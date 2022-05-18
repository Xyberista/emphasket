use std::fs::File;
use std::io::prelude::*;

use reqwest::blocking::Client;

use crate::database;

const TARGET_DIR: &str = "./figures/";

/// Downloads from all available links in the term database
pub fn download() -> Result<(), Box<dyn std::error::Error>> {
    let conn = database::connect()?;
    let terms = database::get_terms(&conn)?;
    conn.close().unwrap();

    let client = Client::new();

    for term in terms {
        if term.picture_link().is_empty() {
            continue;
        }

        let fname = TARGET_DIR.to_string() + &term.picture_name(&term.picture_extension());
        if std::path::Path::new(&fname).exists() {
            continue;
        }

        let target = term.picture_link();
        let response = client.get(target).send()?;

        let mut dest = {
            let extension = response
                .url()
                .path_segments()
                .and_then(|segments| segments.last())
                .and_then(|name| if name.is_empty() { None } else { Some(name) })
                .unwrap_or("tmp.bin")
                .rsplit('.')
                .next()
                .unwrap();
            dbg!(&extension);
            let fname = TARGET_DIR.to_string() + &term.picture_name(extension);
            dbg!(&fname);

            if std::path::Path::new(&fname).exists() {
                continue;
            }

            File::create(fname)?
        };
        let content = response.bytes()?;
        dest.write_all(content.as_ref())?;
    }
    Ok(())
}
