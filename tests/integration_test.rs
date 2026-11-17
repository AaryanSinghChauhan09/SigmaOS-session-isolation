// Integration test for SigmaOS-session-isolation
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use session_isolation as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
