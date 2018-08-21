#![feature(drain_filter)]
#![feature(exact_chunks)]

extern crate byteorder;
#[macro_use] extern crate clap;
#[macro_use] extern crate failure;
extern crate kernel32;
extern crate pancurses;
extern crate rayon;
extern crate read_process_memory;
extern crate winapi;

use clap::{App, Arg};

use failure::Error;

use pancurses::{initscr, endwin, Input, noecho};

mod model;

use model::process::Process;

fn main() -> Result<(), Error> {
    let matches = App::new("Bloodhound Process Searcher")
        .version(crate_version!())
        .author(crate_authors!())
        .arg(Arg::with_name("PID")
            .short("p")
            .long("pid")
            .takes_value(true)
            .required(true)
            .help("The PID of the process to search in."))
        .arg(Arg::with_name("VALUE")
            .short("v")
            .long("value")
            .takes_value(true)
            .required(true)
            .help("Value to start the search with."))
        .get_matches();


    let pid = matches.value_of("PID").unwrap().parse::<u32>()?;
    let value = matches.value_of("VALUE").unwrap().parse::<u32>()?;

    let mut process = Process::new(pid)?;

    let mut search = process.search(value);

    let window = initscr();
    window.keypad(true);
    window.nodelay(true);
    noecho();

    let (y, x) = window.get_max_yx();
    let addresses = window.subwin(y, x / 2, 0, 0).unwrap();

    loop {
        if !search.is_empty() && y > 1 {
            addresses.clear();
            addresses.draw_box(0, 0);
            for i in 0..usize::min(search.len(), y as usize - 2) {
                let value = match search[i].get() {
                    Some(num) => format!("{}", num),
                    None => "???".into(),
                };

                addresses.mvprintw(i as i32 + 1, 1, &format!("{:016X}: {}", search[i].address(), value));
            }
            addresses.refresh();
        }

        window.mvprintw(1, x / 2 + 1, &format!("{} addresses found.", search.len()));

        window.refresh();

        match window.getch() {
            Some(Input::KeyUp) => {
                search.drain_filter(|region| {
                    !region.check(|old, new| new > old)
                });
            }
            Some(Input::KeyDown) => {
                search.drain_filter(|region| {
                    !region.check(|old, new| new < old)
                });
            }
            Some(Input::KeyRight) => {
                search.drain_filter(|region| {
                    !region.check(|old, new| new == old)
                });
            }
            Some(Input::KeyLeft) => {
                search.drain_filter(|region| {
                    !region.check(|old, new| new != old)
                });
            }
            Some(Input::KeyDC) => break,
            Some(Input::KeyIC) => {
            }
            Some(Input::KeyEnter) => {
            }
            Some(_) => (),
            None => (),
        }
    }

    endwin();

    Ok(())
}
