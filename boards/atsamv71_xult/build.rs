// Check memory.x for changes

fn main() {
    println!("cargo::rerun-if-changed=memory.x");
}
