use rusqlite::{params, Connection};
use std::error::Error;
use bunt;
use clap::{clap_app, ArgMatches};
use chrono::NaiveTime;
use std::io;
use std::io::Write;
use std::collections::HashMap;
use serde::Deserialize;
use serde_yaml;
use std::fs::OpenOptions;
use std::fs;
use std::path::Path;

#[cfg(target_family = "unix")]
const DIR: &str = env!("PWD");

#[cfg(target_family = "windows")]
const DIR: &str = env!("CD");

#[derive(Deserialize)]
struct Urls {
    x:  HashMap<String, Vec<String>>,
    cs: HashMap<String, Vec<String>>,
}

#[derive(Debug)]
struct Row {
    series: String,
    episode: u8,
    xtime: Option<u16>,
    ctime: Option<u16>,
    desc: String,
}

fn check_ep(arg: String) -> Result<(), String> {
    arg.parse::<u8>()
        .map(|_| ())
        .map_err(|_| format!("must be a number between 0 and {}", u8::MAX))
}

fn check_time(arg: String) -> Result<(), String> {
    arg.parse::<u16>()
        .map(|_| ())
        .map_err(|_| format!("must be a number between 0 and {}", u16::MAX))
}

fn read_urls() -> Result<Urls, Box<dyn Error>> {
    let urls_path = Path::new(DIR).join("resources").join("urls.yml");
    let urls_raw = fs::read(&urls_path)?;
    let urls_str = String::from_utf8_lossy(&urls_raw);
    Ok(serde_yaml::from_str(&urls_str)?)
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = clap_app!(
        spl =>
        (version: "0.5.1")
        (author: "Jacob Henn")
        (about: "Spl33n moments database tools")
        (
            @subcommand fz =>
            (about: "Fuzzy search the database")
            (@arg TIMEFMT: -t conflicts_with[LINKS] "Prints times as HH:MM:SS")
            (@arg LINKS: -l "Output links to the matching videos")
            (@arg SEARCH: +required "Keyword(s) to search for")
            (@arg SERIES: -s +takes_value "Grabs entries from a single series")
        )
        (
            @subcommand add =>
            (about: "Insert a moment into the database")
            (@arg series: +required "series")
            (@arg episode: {check_ep} +required "episode")
            (
                @arg xtime:
                {check_time} -x +takes_value "X's time (raw seconds)"
            )
            (
                @arg ctime:
                {check_time} -c +takes_value "CS's time (raw seconds)"
            )
            (@arg desc: +required "description")
        )
        (
            @subcommand gencsv =>
            (about: "Generate a moments.csv file")
        )
    ).get_matches();

    let db_path = Path::new(DIR).join("resources").join("spl.db");
    let conn = Connection::open(&db_path)?;

    match matches.subcommand() {
        ("fz",  Some(subm)) => fz(conn, subm),
        ("add", Some(subm)) => add(conn, subm),
        ("gencsv", _) => gencsv(conn),
        (_, None) => Ok(
            bunt::println!("{$bold+red}error:{/$} no subcommand was given")
        ),
        _ => Ok(())
    }
}

fn fz(conn: Connection, subm: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let search = subm.value_of("SEARCH").unwrap();
    
    let query = format!(
        "SELECT {cols} FROM moments WHERE desc LIKE '%{}%'{} ORDER BY {cols}",
        search,
        subm.value_of("SERIES")
            .map(|s| format!(" AND series='{}'", s))
            .unwrap_or_default(),
        cols="series,episode,xtime,ctime,desc",
    );

    let mut stmt = conn.prepare(&query)?;
    let rows = stmt.query_map(params![], |row| {
        Ok(Row {
            series:  row.get(0)?,
            episode: row.get(1)?,
            xtime:   row.get(2)?,
            ctime:   row.get(3)?,
            desc:    row.get(4)?,
        })
    })?;

    let tfmt_arg = subm.is_present("TIMEFMT");
    let link_arg = subm.is_present("LINKS");

    let urls: Option<Urls> =
        if link_arg { Some(read_urls()?) }
        else { None };

    let timefmt = |time: Option<u16>| time.map_or(
        "".into(),
        |t: u16| if tfmt_arg {
            let pat: &[_] = &['0',':'];
            NaiveTime::from_num_seconds_from_midnight(t as u32, 0)
                .to_string()
                .trim_start_matches(pat)
                .to_string()
        } else { t.to_string() }
    );

    let timepad = if tfmt_arg { 7 } else { 4 };

    for row_res in rows {
        let row = row_res?;

        let xtime = timefmt(row.xtime);
        let ctime = timefmt(row.ctime);

        match urls {
            Some(ref u) => {
                let placeholder = String::from("---");
                print!("\n");
                row.xtime.map(|xt| bunt::println!(
                    "{$blue}https://youtu.be/{}&t={}{/$}",
                    u.x.get(&row.series).and_then(
                        |s| s.iter().nth((row.episode - 1) as usize)
                    ).unwrap_or(&placeholder),
                    xt
                )).unwrap_or(());
                row.ctime.map(|ct| bunt::println!(
                    "{$red}https://youtu.be/{}&t={}{/$}",
                    u.cs.get(&row.series).and_then(
                        |s| s.iter().nth((row.episode - 1) as usize)
                    ).unwrap_or(&placeholder),
                    ct
               )).unwrap_or(());
            },
            None => bunt::print!(
                "{[cyan]} {[cyan]:<2} {[bold+cyan]:4$} {[bold+red]:4$} ",
                &row.series,
                &row.episode,
                xtime,
                ctime,
                timepad
            ),
        }

        bunt::println!("{}", &row.desc);
    }
    Ok(())
}

fn add(conn: Connection, subm: &ArgMatches) -> Result<(), Box<dyn Error>> {
    if !subm.is_present("xtime") && !subm.is_present("ctime") {
        bunt::println!("{$bold+red}error:{/$} either {$bold+red}<xtime>{/$}, {$bold+red}<ctime>{/$} or both must be present");
        return Ok(());
    }

    bunt::print!("{$bold+yellow+intense}confirm addition (y/n):{/$} ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    if input.trim() != "y" { return Ok(()); }
    
    conn.execute(
        "INSERT INTO moments VALUES (?1,?2,?3,?4,?5)",
        params![
            subm.value_of("series"),
            subm.value_of("episode"),
            subm.value_of("xtime"),
            subm.value_of("ctime"),
            subm.value_of("desc"),
        ],
    ).map(|_| ())?;
    Ok(())
}

fn gencsv(conn: Connection) -> Result<(), Box<dyn Error>> {
    let mut stmt = conn.prepare("SELECT * FROM moments")?;
    let rows = stmt.query_map(params![], |row| {
        Ok(Row {
            series:  row.get(0)?,
            episode: row.get(1)?,
            xtime:   row.get(2)?,
            ctime:   row.get(3)?,
            desc:    row.get(4)?,
        })
    })?;

    let urls = read_urls()?;

    let csv = OpenOptions::new()
        .append(true)
        .create(true)
        .open("moments.csv")?;

    for row_res in rows {
        let row = row_res?;
        let placeholder = String::new();

        let xtime = &row.xtime.map(|x|
            NaiveTime::from_num_seconds_from_midnight(x as u32, 0)
                .to_string()
        ).unwrap_or_default();

        let ctime = &row.ctime.map(|c|
            NaiveTime::from_num_seconds_from_midnight(c as u32, 0)
                .to_string()
        ).unwrap_or_default();

        writeln!(
            &csv, r#""{}",{},{},{},{},{},{}"#,
            &row.desc,
            &row.series,
            &row.episode,
            xtime, ctime,
            row.xtime.map(|xt| format!(
                "https://youtu.be/{}&t={}",
                urls.x.get(&row.series).and_then(
                    |s| s.iter().nth((row.episode - 1) as usize)
                ).unwrap_or(&placeholder),
                xt
            )).unwrap_or_default(),
            row.ctime.map(|ct| format!(
                "https://youtu.be/{}&t={}",
                urls.cs.get(&row.series).and_then(
                    |s| s.iter().nth((row.episode - 1) as usize)
                ).unwrap_or(&placeholder),
                ct
           )).unwrap_or_default(),
        )?;
    }

    Ok(())
}
