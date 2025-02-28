use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::time::Instant;

//Our simple bank account struct
pub struct Account {
    pub id: i32,
    pub balance: i32,
}

// A  function that attempts to transfer funds between two accounts.
// We us try_lock here because it is a non blocking lock function which is prone to deadlocks.
pub fn transfer(from: &Arc<Mutex<Account>>, to: &Arc<Mutex<Account>>, amount: i32) -> bool {
    let thread_id = thread::current().id(); //logging for debuggingx
                                            // Start the timer for detecting deadlock.
    let start = Instant::now();
    loop {
        //Ordering our accounts to prevent deadlocks
        let from_id = from.lock().unwrap().id;

        let to_id = to.lock().unwrap().id;

        //this condition ensures that no two threads will have each others locks
        //by locking the smaller id account frist every time all the threads will follow the same pattern
        let (first, second) = if from_id < to_id {
            (from, to)
        } else {
            (to, from)
        };

        let first_lock = first.try_lock();
        let second_lock = second.try_lock();

        match (first_lock, second_lock) {
            // If both locks are acquired, perform the transfer. Also checking to see if there is enough balance to cover the transfer
            (Ok(mut first_guard), Ok(mut second_guard)) => {
                if first_guard.id == from_id {
                    if first_guard.balance >= amount {
                        first_guard.balance -= amount;
                        second_guard.balance += amount;
                        println!(
                            "Thread {:?}, Transferred {} successfully!",
                            thread_id, amount
                        );
                    } else {
                        println!(
                            "Thread {:?} failed, Insufficient funds for transfer.",
                            thread_id
                        );
                    }
                } else {
                    if second_guard.balance >= amount {
                        second_guard.balance -= amount;
                        first_guard.balance += amount;
                        println!(
                            "Thread {:?}, Transferred {} successfully!",
                            thread_id, amount
                        );
                    } else {
                        println!(
                            "Thread {:?} failed, Insufficient funds for transfer.",
                            thread_id
                        );
                    }
                }
                return true;
            }
            // If either lock fails, check if we have waited too long.
            _ => {
                if start.elapsed() > Duration::from_secs(1) {
                    println!("Deadlock detected! Aborting transfer of {}.", amount);
                    return false;
                }
                // Pause and try again.
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
    //loop 10 times
    for n in 1..10 {
        // Spawn a thread that tries to transfer money from account1 to account2.
        let acc1 = Arc::clone(&account1);
        let acc2 = Arc::clone(&account2);
        let thread1 = thread::spawn(move || {
            println!("Thread 1: Transferring 100 from Account1 to Account2");
            if !transfer(&acc1, &acc2, 100) {
                println!("Thread 1: Transfer failed due to deadlock.");
            }
        });

        // Spawn another thread that tries to transfer money from account2 to account1.
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

        // Display final balances.
        println!(
            "Final balances -> Account1: {}, Account2: {}",
            account1.lock().unwrap().balance,
            account2.lock().unwrap().balance
        );
        //repeating after 10 milis
        println!("Pausing for 1 second...");
        thread::sleep(Duration::from_millis(10));
        println!("Resuming execution.");
    }
}
