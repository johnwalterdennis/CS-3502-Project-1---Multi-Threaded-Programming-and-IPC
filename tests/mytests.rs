#[cfg(test)]
mod tests {
    use super::*; // Import everything from the current module
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    use Project1Phase1::{transfer, Account};

    // -------------------------------------------------------
    // 1. Concurrency Test
    // -------------------------------------------------------
    // Goal: Verify multiple threads can safely run transfer
    //       at the same time without data races.
    //
    // Scenario: We create two accounts, then spawn several
    //           threads each transferring funds between them.
    //           We finally assert the sum of the balances is
    //           unchanged and no corruption occurred.
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

        // Spawn multiple threads
        let mut handles = vec![];
        for _ in 0..5 {
            let acc1_clone = Arc::clone(&account1);
            let acc2_clone = Arc::clone(&account2);
            handles.push(thread::spawn(move || {
                // Each thread calls transfer several times
                for _ in 0..10 {
                    transfer(&acc1_clone, &acc2_clone, 10);
                }
            }));
        }

        // Wait for all threads to finish
        for handle in handles {
            handle.join().unwrap();
        }

        // Validate final balances
        let final_balance1 = account1.lock().unwrap().balance;
        let final_balance2 = account2.lock().unwrap().balance;
        assert_eq!(
            final_balance1 + final_balance2,
            2000,
            "Total balance should remain 2000"
        );
    }

    // -------------------------------------------------------
    // 2. Synchronization / Deadlock Test
    // -------------------------------------------------------
    // Goal: Confirm that `transfer` prevents deadlocks using
    //       the try_lock logic and timeout.
    //
    // Scenario: Two threads simultaneously attempt to transfer
    //           in opposite directions. If your code’s locking
    //           approach and timeout logic works, it should
    //           either succeed in transferring without deadlock
    //           or abort one transfer if it detects a deadlock.
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
            // Transfer from account1 to account2
            let result = transfer(&acc1_clone_a, &acc2_clone_a, 500);
            println!("Thread 1 transfer result: {}", result);
        });

        let acc1_clone_b = Arc::clone(&account1);
        let acc2_clone_b = Arc::clone(&account2);
        let handle2 = thread::spawn(move || {
            // Transfer from account2 to account1
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

        // If your code prints "Deadlock detected! Aborting transfer...",
        // it's still fine as long as total is correct and no indefinite
        // block occurred.
    }

    // -------------------------------------------------------
    // 3. Stress Test
    // -------------------------------------------------------
    // Goal: Test stability and performance under heavy load.
    //
    // Scenario: High number of threads each performing multiple
    //           transfers. The system should avoid deadlock,
    //           remain responsive, and maintain correct balances.
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

        // Example: 50 threads, each doing multiple transfers
        for _ in 0..50 {
            let acc1_clone = Arc::clone(&account1);
            let acc2_clone = Arc::clone(&account2);
            handles.push(thread::spawn(move || {
                for _ in 0..100 {
                    // Transfer a small random or fixed amount
                    // Here we just use 10 as an example
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

        // The total should still be 20,000
        assert_eq!(
            final_balance1 + final_balance2,
            20_000,
            "Total balance should remain 20,000 after stress test"
        );
    }

    // -------------------------------------------------------
    // 4. Insufficient Funds Test
    // -------------------------------------------------------
    // Goal: Demonstrate how transfer handles an account that
    //       lacks enough balance.
    //
    // Scenario: Attempt to transfer more than the `from` account
    //           has. The code logs "Insufficient funds" and does
    //           not change balances.
    #[test]
    fn test_insufficient_funds() {
        let account1 = Arc::new(Mutex::new(Account {
            id: 1,
            balance: 300,
        }));
        let account2 = Arc::new(Mutex::new(Account {
            id: 2,
            balance: 500,
        }));

        // Try to transfer 1000 from account1 to account2 (which only has 300)
        let result = transfer(&account1, &account2, 1000);
        assert_eq!(
            result, true,
            "Function returns true if it acquired the locks, even though funds are insufficient."
        );

        // Now check that the balances are unchanged (since transfer should fail logic-wise)
        let final_balance1 = account1.lock().unwrap().balance;
        let final_balance2 = account2.lock().unwrap().balance;
        assert_eq!(
            final_balance1, 300,
            "Account1 balance should remain 300 if transfer fails"
        );
        assert_eq!(
            final_balance2, 500,
            "Account2 balance should remain 500 if transfer fails"
        );
    }
}
