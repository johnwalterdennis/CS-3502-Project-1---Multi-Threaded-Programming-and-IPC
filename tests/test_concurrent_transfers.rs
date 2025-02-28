#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Instant;
    // Import our functions from the current module
    use Project1Phase1::{transfer, Account};

    // We create two accounts, then spawn several
    // threads transferring funds between them.
    // the sum of the amount between both accounts should be the same as they were at the start
    #[test]
    fn test_concurrent_transfers() {
        let start = Instant::now();
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
        let duration = start.elapsed();
        assert_eq!(
            final_balance1 + final_balance2,
            2000,
            "Total balance should remain 2000"
        );
        println!("Test executed in: {:?}", duration);
    }
}
