# djpass (Djando Passwords)

A command-line tool to generate and verify passwords used in [Django Project](https://www.djangoproject.com).

## Installation

You can compile from source, or install via `cargo` (requires version 0.6.0 or higher):

```
$ cargo install djpass
```

## Usage

### Generating a Password Hash

If you run without arguments a password will be asked:

```
$ djpass
Password: [password]
Hash: pbkdf2_sha256$24000$...
```

Passing the password as a single argument will return the hash:

```
$ djpass hello
Hash: pbkdf2_sha256$24000$...
```

You can also specify the algorithm used:

```
$ djpass hello -a sha1
Hash: sha1$hzPiRIKYykm8$23...
```

Algorithms:

- `PBKDF2` (default)
- `PBKDF2SHA1`
- `BCryptSHA256`
- `BCrypt`
- `SHA1`
- `MD5`
- `UnsaltedSHA1`
- `UnsaltedMD5`

The algorithm argument is case-insensitive.

### Verifying a Password Hash

```
$ djpass hello 'sha1$hzPiRIKYykm8$23...'
Password ok.
```

If you pass an algorithm during verification it will be ignored.


## Contributing

* Be polite, I’m new to Rust, like almost everybody.
* Don't go nuts with your *mad-rust-skillz*, legibility is a priority.
* Always include some test case.

## License

Djpass is released under the **3-Clause BSD License**.

**tl;dr**: *"free to use as long as you credit me"*.
