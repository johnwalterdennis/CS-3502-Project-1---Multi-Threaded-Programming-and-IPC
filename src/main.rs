use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::time::Instant;

struct Account {
    id: i32,
    balance: i32,
}

// A helper function that attempts to transfer funds between two accounts.
// It uses try_lock in a loop with a timeout to detect potential deadlock.
fn transfer(from: &Arc<Mutex<Account>>, to: &Arc<Mutex<Account>>, amount: i32) -> bool {
    // Start the timer for detecting deadlock.
    let start = Instant::now();
    loop {
        // Attempt to lock the "from" account without blocking.
        let from_id = from.lock().unwrap().id;
        // Attempt to lock the "to" account without blocking.
        let to_id = to.lock().unwrap().id;

        let (first, second) = if from_id < to_id {
            (from, to)
        } else {
            (to, from)
        };

        let first_lock = first.try_lock();
        let second_lock = second.try_lock();

        match (first_lock, second_lock) {
            // If both locks are acquired, perform the transfer.
            (Ok(mut first_guard), Ok(mut second_guard)) => {
                if first_guard.id == from_id {
                    if first_guard.balance >= amount {
                        first_guard.balance -= amount;
                        second_guard.balance += amount;
                        println!("Transferred {} successfully!", amount);
                    } else {
                        println!("Insufficient funds for transfer.");
                    }
                } else {
                    if second_guard.balance >= amount {
                        second_guard.balance -= amount;
                        first_guard.balance += amount;
                        println!("Transferred {} successfully!", amount);
                    } else {
                        println!("Insufficient funds for transfer.");
                    }
                }
                return true;
            }
            // If either lock fails, check if we have waited too long.
            _ => {
                if start.elapsed() > Duration::from_secs(1) {
                    // Deadlock detected after waiting 1 second.
                    println!("Deadlock detected! Aborting transfer of {}.", amount);
                    return false;
                }
                // Pause briefly and try again.
                thread::sleep(Duration::from_millis(10));
            }
        }
    }
}

fn main() {
    // Create two bank accounts, each with an initial balance.
    let account1 = Arc::new(Mutex::new(Account {
        id: 1,
        balance: 1000,
    }));
    let account2 = Arc::new(Mutex::new(Account {
        id: 2,
        balance: 1000,
    }));

    loop {
        // Spawn a thread that attempts to transfer money from account1 to account2.
        let acc1 = Arc::clone(&account1);
        let acc2 = Arc::clone(&account2);
        let thread1 = thread::spawn(move || {
            println!("Thread 1: Transferring 100 from Account1 to Account2");
            if !transfer(&acc1, &acc2, 100) {
                println!("Thread 1: Transfer failed due to deadlock.");
            }
        });

        // Spawn another thread that attempts to transfer money from account2 to account1.
        let acc1 = Arc::clone(&account1);
        let acc2 = Arc::clone(&account2);
        let thread2 = thread::spawn(move || {
            println!("Thread 2: Transferring 200 from Account2 to Account1");
            if !transfer(&acc2, &acc1, 200) {
                println!("Thread 2: Transfer failed due to deadlock.");
            }
        });

        // Wait for both threads to finish.
        thread1.join().unwrap();
        thread2.join().unwrap();

        // Display final balances in both accounts.
        println!(
            "Final balances -> Account1: {}, Account2: {}",
            account1.lock().unwrap().balance,
            account2.lock().unwrap().balance
        );
        println!("Pausing for 1 second...");
        thread::sleep(Duration::from_secs(1));
        println!("Resuming execution.");
    }
}

// fn main() {
//     let account = Arc::new(Mutex::new(1000)); // Starting with a balance of 1000
//     let mut handles = vec![];

//     for i in 0..5 {
//         let account_clone = Arc::clone(&account);
//         let handle = thread::spawn(move || {
//             let mut balance = account_clone.lock().unwrap();
//             println!("Thread {} acquired the lock.", i);
//             *balance += 1000;
//             println!("Thread {} deposited 1000, New balance is {}", i, *balance);
//         });
//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("Final account balance: {}", *account.lock().unwrap());
// }
