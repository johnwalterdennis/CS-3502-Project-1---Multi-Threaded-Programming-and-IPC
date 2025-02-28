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
    fn test_stress_transfers() {
        let start = Instant::now();
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
        for _ in 0..50 {
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
        let duration = start.elapsed();
        println!("Test executed in: {:?}", duration);
    }
}
