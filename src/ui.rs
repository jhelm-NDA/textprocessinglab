// ============================================================
// UI Module (No TODOs)
// ============================================================

pub fn show(title: &str, items: &[String]) {
    println!("--- {} ---", title);
    for item in items {
        println!("{}", item);
    }
}
