# CS-3502-Project-1---Multi-Threaded-Programming-and-IPC

This project is divided into parts.
Phase 1 which is a set of programs and tests demonstrating your understanding of threading concepts. By implementing the creation of safe
and synchronizationable threads, safe resource sharing with mutexes, deadlock detection, recovery and prevention practices in rust.
Phase 1 uses the example of a hypotethical Banking system with threads concurrently making transfers between accounts. Spawning several threads and transferring a simple amount a series of times.
Test cases involve testing for concurrent transfers, deadlock prevention and stress testing by spawing 100 active threads.

## Installation

The only dependency necessary to run all of the code in this program is rust and the rust compiler, to dowload.

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
Thread 1: Transferring 100 from Account1 to Account2
Thread ThreadId(2), Transferred 100 successfully!
Thread 2: Transferring 200 from Account2 to Account1
Thread ThreadId(3), Transferred 200 successfully!
Final balances -> Account1: 1100, Account2: 900
Pausing for 1 second...
Resuming execution.
Thread 1: Transferring 100 from Account1 to Account2
Thread ThreadId(4), Transferred 100 successfully!
Thread 2: Transferring 200 from Account2 to Account1
Thread ThreadId(5), Transferred 200 successfully!
Final balances -> Account1: 1200, Account2: 800
Pausing for 1 second...

# cargo test
running 3 tests
test tests::test_concurrent_transfers ... ok
test tests::test_deadlock_prevention ... ok
test tests::test_stress_transfers ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 24.15s

```

See [output.txt](https://github.com/johnwalterdennis/CS-3502-Project-1---Multi-Threaded-Programming-and-IPC/blob/phase1/output.txt) files for more output examples.

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[GNU](https://www.gnu.org/licenses/gpl-3.0.en.html)
