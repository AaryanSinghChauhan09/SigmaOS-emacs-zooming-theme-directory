//! SigmaOS-emacs-zooming-theme-directory — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_emacs_zooming_theme_directory_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-emacs-zooming-theme-directory: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-emacs-zooming-theme-directory: ok");
}
