use std::thread;

fn main() {
    // thread::spawn takes a closure with no arguments
    let handle = thread::spawn(move || {
        // This closure is executed as the main function of the thread
        // Do stuff in a child thread
    });

    // Do stuff simultaneously in the main thread

    // Wait until thread has exited
    // Spawn returns a JoinHandle. We can use it to call join() which will pause the thread that we're on until
    // The thread we're joining has completed and exited
    handle.join().unwrap();

    // The more threads you have trying to share a CPU core, the more overhead you will have in context switching
}
