use nix::sys::wait::wait;
use nix::unistd::{close, fork, pipe, read, write, ForkResult};
use std::process::exit;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// producer and consumer process
///
/// - The producewr writes data into the pipe.
/// - The consumer reads data, processes it (uppercase transformation), and returns the result.
/// the consumer should read the data from the procuder, change it from lower case to uppercase and print the data
pub fn run_ipc_pipe(
    input_messages: Vec<&str>,
    break_pipe: Arc<AtomicBool>,
) -> Result<String, String> {
    // Create pipe
    let (fd_read, fd_write) = match pipe() {
        Ok(fds) => fds,
        Err(e) => return Err(format!("Failed to create pipe: {}", e)),
    };

    // Fork process
    let fork_result = unsafe { fork() };

    match fork_result {
        Ok(ForkResult::Child) => {
            close(fd_write).expect("Consumer: Failed to close write end");

            let mut buffer = [0u8; 1024];
            let mut output = Vec::new();

            while let Ok(n) = read(fd_read, &mut buffer) {
                if n == 0 {
                    break;
                }
                for i in 0..n {
                    if buffer[i].is_ascii_lowercase() {
                        buffer[i] = buffer[i].to_ascii_uppercase();
                    }
                }
                output.extend_from_slice(&buffer[..n]);
            }

            close(fd_read).expect("Consumer: Failed to close read end");

            // Convert to UTF-8 string and return via exit code
            let processed_output = String::from_utf8_lossy(&output).to_string();
            println!(
                "Consumer: Processed the following text to uppercase: {}",
                processed_output
            );
            exit(0);
        }

        Ok(ForkResult::Parent { .. }) => {
            close(fd_read).expect("Parent: Failed to close read end");

            // If break_pipe is true, close the write end prematurely
            // before writing any data
            if break_pipe.load(Ordering::SeqCst) {
                close(fd_write).expect("Parent: Deliberately closing write end early");
                wait().expect("Failed to wait for child");
                return Err("Pipe was deliberately broken".to_string());
            }

            for msg in &input_messages {
                println!("Producer sent the following messages: {}\n", msg);
                if write(fd_write, msg.as_bytes()).is_err() {
                    return Err("Parent: Failed to write data".to_string());
                }
            }
            close(fd_write).expect("Parent: Failed to close write end");
            wait().expect("Failed to wait for child");
            Ok("[Parent] Child process exited successfully".to_string())
        }
        Err(e) => Err(format!("Fork failed: {}", e)),
    }
}

/// Main function for manual testing
fn main() {
    let break_pipe = Arc::new(AtomicBool::new(false));
    let input = vec!["hello world!\n", "this is a test.\n"];
    match run_ipc_pipe(input, Arc::clone(&break_pipe)) {
        Ok(msg) => println!("{}", msg),
        Err(err) => eprintln!("{}", err),
    }
}
