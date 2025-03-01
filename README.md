# CS-3502-Project-1---Multi-Threaded-Programming-and-IPC

This project is divided into parts.
Phase 2 which is a of program and tests demonstrating an understanding how processes communicate through pipes.
This program involves a custom program running a basic producer-consumer pipeline between two functions using the powerful rust pipe() and fork() modules.
It starts with a producer process that passes a simple text string into a unidirectional pipe, a consumer process then picks up the process and converts the string into uppercase and prints the result to `stdout` and prints it out to the screen.
Test cases involve testing for data integrity, broken pipe recovery and large data transfers

## Installation

The only dependency necessary to run all of the code in this program is rust and the rust compiler. To dowload;

Mac:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Windows and others: [see offical rust website](https://www.rust-lang.org/tools/install)

## Usage

Initialize local repository, clone directory into local repository. Navigate to local directory.

```rust
cargo run

# returns something like:
Consumer: Processed the following text to uppercase: HELLO WORLD!
THIS IS A TEST.

[Parent] Child process exited successfully


# cargo test
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 3 tests
test test_broken_pipe ... ok
test test_data_integrity ... FAILED
test test_large_data_performance ... ok

failures:

---- test_data_integrity stdout ----
Expected: Consumer: Processed the following text to uppercase: {        "USER": {            "ID": 12345,            "NAME": "BRYAN USER",            "EMAIL": "BRYAN@EXAMPLE.COM"            }        }Producer: consumer process exited successfully
Actual: [Parent] Child process exited successfully
thread 'test_data_integrity' panicked at tests/mytest.rs:28:5:
assertion failed: result.contains(&json_doc_uppercase1)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

```

See [output.txt](https://github.com/johnwalterdennis/CS-3502-Project-1---Multi-Threaded-Programming-and-IPC/blob/phase2/output.txt) files for more output examples.

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[GNU](https://www.gnu.org/licenses/gpl-3.0.en.html)
