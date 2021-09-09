// the Send marker trait indicates that ownership of values of the type can be
// transferred between threads; types composed entirely of other types marked
// as Send will be marked as Send as well

// the Sync marker trait indicates that that it is safe to reference the type
// from multiple threads, e.g. Mutex<T>

// implementing either of these traits is unsafe

pub fn run() {
    // oh, there's actually no code here
}