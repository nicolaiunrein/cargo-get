use std::error::Error;

use clap::ArgMatches;

use crate::delimiter::Delimiter;

pub fn match_version(
    matches: &ArgMatches,
    v: semver::Version,
    delimiter: &Delimiter,
) -> Result<(), Box<dyn Error>> {
    if matches.is_present("full") {
        println!("{}", v);
        return Ok(());
    }

    if matches.is_present("pretty") {
        println!("v{}", v);
        return Ok(());
    }

    let mut out = Vec::new();
    let delim_string = delimiter.to_string();

    if matches.is_present("major") {
        out.push(v.major.to_string());
    }

    if matches.is_present("minor") {
        out.push(v.minor.to_string());
    }

    if matches.is_present("patch") {
        out.push(v.patch.to_string())
    }

    if matches.is_present("build") {
        for b in v.build.iter() {
            out.push(format!("{}", b))
        }
    }
    if matches.is_present("pre") {
        for p in v.pre.iter() {
            out.push(format!("{}", p))
        }
    }
    if out.is_empty() {
        out.push(format!("{}", v));
    }
    println!("{}", out.join(&delim_string));
    return Ok(());
}
