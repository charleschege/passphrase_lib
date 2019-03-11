//! Passhrase is a pure Rust crate focused on generating solid passphrases at the touch of a button
//! , giving users the ability to generate memorable strong passphrases that takes thousands of
//! years to crack using specialized password cracking computers or taking forever for normal
//! computers to crack.
//!
//! At the moment(an English only version is coming soon), it combines English and Swahili
//! dictionaries of short easy to type words.
//! The `zxcvbn` crate, a password strength estimator based off of Dropbox's zxcvbn library,
//! is used to calculate how long it would take to crack the password, the number of guesses it
//! would need and the number of years it would take to crack the passphrase.
//!
//! However, kindly node that this is only a passphrase generator and you need to hash it with
//! a good hashing algorithm (I recommend argon2 implementations) for use to store in a database
//!
//! [WARNING!!!] NEVER STORE THE PLAIN TEXT VERSION OF A PASSPHRASE
//!
//! use `cargo install passphrase` to install it on your machine.
//!
//! Run it via the commandline as
//!
//! $ `passhrase`

mod dictionary;

use {
    std::io::{Write, stdout, stdin},
    rand::{Rng, thread_rng},
    termion::{
        cursor,
        raw::IntoRawMode,
        event::Key,
        input::TermRead,
        clear,
    },
    yansi::{Paint},
    zxcvbn::zxcvbn,
    crate::dictionary::*,
};

fn choose_a_word(data_type: &'static[&'static str]) -> &str {
    let size = data_type.len();
    let data_type = data_type[thread_rng().gen_range(0, size)];
    data_type
}

pub fn gen_passphrase() -> (String, u8, u64, String){
    let first_iteration = SWAHILI;
    let second_iteration = ENGLISH;
    let third_iteration = SWAHILI;
    let fourth_iteration = ENGLISH;

    let passphr = format!("{} {} {} {}", choose_a_word(second_iteration), choose_a_word(first_iteration),  choose_a_word(fourth_iteration), choose_a_word(third_iteration));
    let estimate = zxcvbn(&passphr, &[]).unwrap();

    (
        passphr,
        estimate.score,
        estimate.guesses,
        estimate.crack_times_display.offline_slow_hashing_1e4_per_second,

    )

}

fn strength_to_str(score: u8) -> &'static str {
    match score {
        1 => "VERY POOR",
        2 => "POOR",
        3 => "GOOD",
        4 => "STRONG",
        _ => "ERROR",
    }
}

fn to_stdout() {

    let mut stdout = stdout().into_raw_mode().unwrap();
    let passphr = gen_passphrase();
    {
        writeln!(stdout, "{}{}→! PASSPHRASE: {}\n",
            clear::All,
            cursor::Goto(4,3),
            Paint::green(passphr.0)
        ).unwrap();

        writeln!(stdout, "{}{}→ The strength of this passphrase is {}",
            clear::UntilNewline,
            cursor::Goto(4,6),
            Paint::cyan(strength_to_str(passphr.1)),
        ).unwrap();

        writeln!(stdout, "{}{}→ {} guesses needed to crack passphrase.",
            clear::UntilNewline,
            cursor::Goto(4,7),
            Paint::yellow(passphr.2)
        ).unwrap();

        writeln!(stdout, "{}{}→ It would take {} to crack this passphrase.",
            clear::UntilNewline,
            cursor::Goto(4,8),
            Paint::green(passphr.3)
        ).unwrap();
    }

    writeln!(stdout, "{}{}[ Press `N` for Next Passphrase]. Use `Esc` to exit.\n",
        clear::UntilNewline,
        cursor::Goto(3,12),
    ).unwrap();

    stdout.flush().unwrap();
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    writeln!(stdout, "{}{}{}",
        clear::All,
        cursor::Goto(1,1),
        cursor::Hide,
    ).unwrap();

    to_stdout();

    for c in stdin.keys() {
        write!(stdout, "{}{}", cursor::Goto(4,3) ,clear::CurrentLine).unwrap();

        match c.unwrap() {
            Key::Esc => {
                println!("{} {}",
                Paint::yellow("I hope you use me soon."),
                Paint::green("Bye now!"),
            );
                break
            },
            Key::Char('n') => {
                to_stdout();
            },
            Key::Char('N') => {
                to_stdout();
            },
            Key::Ctrl('c') => {
                println!("{} {}",
                    Paint::red("→ USER INTERRUPTED THE PROGRAM FORCEFULLY!"),
                    Paint::green("[PROCESS EXIT CODE: 0]"),
                );
                std::process::exit(0)
            },
            _ => writeln!(stdout, "{}{}{}\n",
                clear::All,
                cursor::Goto(3,2),
                Paint::red("[ Press `N` for Next Passphrase]. Use `Esc` to exit."),
            ).unwrap(),
        }
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", cursor::Show).unwrap();
}
