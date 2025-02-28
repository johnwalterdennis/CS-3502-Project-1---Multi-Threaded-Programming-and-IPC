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
    fn test_deadlock_prevention() {
        let start = Instant::now();
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
        let duration = start.elapsed();
        println!("Test executed in: {:?}", duration);
        // if the code detects a deadlock and aborts the transfer then the final amounts and inital amounts will be the same,
        //
    }
}
