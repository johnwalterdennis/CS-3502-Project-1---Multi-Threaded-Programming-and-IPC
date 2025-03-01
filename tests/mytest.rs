use ::project1phase2::run_ipc_pipe;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

//this test will feed in a json document as its input string and compare the output from out function
#[test]
fn test_data_integrity() {
    let json_doc = r#"{
        "user": {
            "id": 12345,
            "name": "bryan user",
            "email": "bryan@example.com"
        }"#;
    let json_doc_uppercase = r#"Consumer: Processed the following text to uppercase: {
        "USER": {
            "ID": 12345,
            "NAME": "BRYAN USER",
            "EMAIL": "BRYAN@EXAMPLE.COM"
            }
        }
Producer: consumer process exited successfully"#;
    let json_doc_uppercase1 = json_doc_uppercase.replace("\n", "").replace("\r", "");
    let input_messages = vec![json_doc];
    let break_pipe = Arc::new(AtomicBool::new(false));
    let result = run_ipc_pipe(input_messages, Arc::clone(&break_pipe)).expect("IPC failed");
    eprintln!("Expected: {}", json_doc_uppercase1);
    eprintln!("Actual: {}", result);
    assert!(result.contains(&json_doc_uppercase1));
}

// this test case will intentionally break the pipe by passing in the Arc::new(AtomicBool::new(true)) into our function
// the test case will resolve as true if the result of that is not an error which means our function  successfully aborted a broken pipe
#[test]
fn test_broken_pipe() {
    let input_messages = vec!["hello world\n", "test message\n"];
    let break_pipe = Arc::new(AtomicBool::new(true));
    // Run IPC but close the pipe early
    let result = run_ipc_pipe(input_messages, Arc::clone(&break_pipe));

    assert!(
        result.is_err(),
        "Expected an error when the pipe is closed prematurely"
    );
}

//this test case will input are large string of 10 MBs of of just the letter A and make sure that the
// program runs in under 2 seconds
#[test]
fn test_large_data_performance() {
    use std::time::Instant;
    let break_pipe = Arc::new(AtomicBool::new(false));

    let binding = "A".repeat(10_000_000);
    let large_input = vec![binding.as_str()];
    let start_time = Instant::now();

    let result = run_ipc_pipe(large_input, Arc::clone(&break_pipe));

    let elapsed_time = start_time.elapsed();
    assert!(
        elapsed_time.as_secs_f64() < 2.0,
        "Data transfer took too long"
    );
    assert!(result.is_ok());
}
