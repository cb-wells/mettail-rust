// Build script for mettail-runtime
// Processes LALRPOP grammar files into Rust parsers

fn main() {
    lalrpop::process_root().unwrap();
}

