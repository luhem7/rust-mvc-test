use std::{
    thread,
    sync::{Arc,Mutex},
    time::Duration,
};

struct View;

impl View {
    fn new() -> Self {
        Self
    }

    fn render_view(&self) {
        println!("Rendering View");
    }
}

fn main() {
    println!("Initialized App.");

    let view_arc = Arc::new(Mutex::new(View::new()));
    let view_arc_clone = Arc::clone(&view_arc);
}
