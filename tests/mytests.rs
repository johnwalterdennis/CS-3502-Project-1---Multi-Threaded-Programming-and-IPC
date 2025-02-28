#[cfg(test)]
mod tests {
    use super::*; // Import everything from the current module
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    use Project1Phase1::{transfer, Account};

    // -------------------------------------------------------
    // Testing our functions
    // -------------------------------------------------------

    // We create two accounts, then spawn several
    // threads transferring funds between them.
    // the sum of the amount between both accounts should be the same as they were at the start
    #[test]
    fn test_concurrent_transfers() {
        let account1 = Arc::new(Mutex::new(Account {
            id: 1,
            balance: 1000,
        }));
        let account2 = Arc::new(Mutex::new(Account {
            id: 2,
            balance: 1000,
        }));

        // Spawn 5 threads
        let mut handles = vec![];
        for _ in 0..5 {
            let acc1_clone = Arc::clone(&account1);
            let acc2_clone = Arc::clone(&account2);
            handles.push(thread::spawn(move || {
                // Each thread calls transfer 10 times
                for _ in 0..10 {
                    transfer(&acc1_clone, &acc2_clone, 10);
                }
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        // check final balances
        let final_balance1 = account1.lock().unwrap().balance;
        let final_balance2 = account2.lock().unwrap().balance;
        assert_eq!(
            final_balance1 + final_balance2,
            2000,
            "Total balance should remain 2000"
        );
    }

    // we intentionally introduce deadlock conditions by performing opposing transfers on two threads
    //if there is no deadlock then the transfer will either resolve or timeout provign that no race conditions has occured
    #[test]
    fn test_deadlock_prevention() {
        let account1 = Arc::new(Mutex::new(Account {
            id: 1,
            balance: 1000,
        }));
        let account2 = Arc::new(Mutex::new(Account {
            id: 2,
            balance: 1000,
        }));

        let acc1_clone_a = Arc::clone(&account1);
        let acc2_clone_a = Arc::clone(&account2);
        let handle1 = thread::spawn(move || {
            let result = transfer(&acc1_clone_a, &acc2_clone_a, 500);
            println!("Thread 1 transfer result: {}", result);
        });

        let acc1_clone_b = Arc::clone(&account1);
        let acc2_clone_b = Arc::clone(&account2);
        let handle2 = thread::spawn(move || {
            let result = transfer(&acc2_clone_b, &acc1_clone_b, 300);
            println!("Thread 2 transfer result: {}", result);
        });

        handle1.join().unwrap();
        handle2.join().unwrap();

        // Final check: total balance must remain 2000.
        let final_balance1 = account1.lock().unwrap().balance;
        let final_balance2 = account2.lock().unwrap().balance;
        assert_eq!(
            final_balance1 + final_balance2,
            2000,
            "Total balance should remain 2000 after opposite transfers"
        );

        // if the code detects a deadlock and aborts the transfer then the final amounts and inital amounts will be the same,
        //
    }

    //
    // we create a High number of threads each performing multiple
    //  transfers.
    #[test]
    fn test_stress_transfers() {
        let account1 = Arc::new(Mutex::new(Account {
            id: 1,
            balance: 10_000,
        }));
        let account2 = Arc::new(Mutex::new(Account {
            id: 2,
            balance: 10_000,
        }));

        let mut handles = vec![];

        //100 threads, each doing 100 transfers
        for _ in 0..100 {
            let acc1_clone = Arc::clone(&account1);
            let acc2_clone = Arc::clone(&account2);
            handles.push(thread::spawn(move || {
                for _ in 0..100 {
                    transfer(&acc1_clone, &acc2_clone, 10);
                }
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        // Check final balances
        let final_balance1 = account1.lock().unwrap().balance;
        let final_balance2 = account2.lock().unwrap().balance;

        // The total should still remain the same
        assert_eq!(
            final_balance1 + final_balance2,
            20_000,
            "Total balance should remain 20,000 after stress test"
        );
    }
}
