## passphrase_lib

Passhrase is a pure Rust crate focused on generating solid passphrases at the touch of a button , giving users the ability to generate memorable strong passphrases that takes thousands of years to crack using specialized password cracking computers or taking forever for normal computers to crack.

At the moment(an English only version is coming soon), it combines English and Swahili dictionaries of short easy to type words.
The `zxcvbn` crate, a password strength estimator based off of Dropbox's zxcvbn library, has been used to counter-check how long it would take to crack the password, the number of guesses it would need and the number of years it would take to crack the passphrase.

However, kindly node that this is only a passphrase generator and you need to hash it with a good hashing algorithm (I recommend argon2 implementations) for use to store in a database

**WARNING!!!** NEVER STORE THE PLAIN TEXT VERSION OF A PASSPHRASE

---

use `cargo add passphrase` to install to add to your `Cargo.toml` file.

To add it manually to `Cargo.toml` file.
```
[dependencies]
passhrase_lib = #enter the version from crates.io here
```
---