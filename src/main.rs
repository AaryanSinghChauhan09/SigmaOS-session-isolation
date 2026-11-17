//! SigmaOS-session-isolation — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_session_isolation_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-session-isolation: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-session-isolation: ok");
}
