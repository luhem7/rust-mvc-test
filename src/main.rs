use std::{
    thread,
    sync::{Arc,Mutex},
    time::Duration,
};

struct View {
    counter: i32
}

impl View {
    fn new() -> Self {
        Self {
            counter: 0
        }
    }

    fn render_view(&mut self) {
        println!("Rendering View {}", self.counter);
        self.counter += 1;
    }
}

fn main() {
    println!("Initialized App.");

    let view_arc = Arc::new(Mutex::new(View::new()));
    let view_arc_clone = Arc::clone(&view_arc);

    loop {
        let mut view = view_arc_clone.lock().unwrap();
        view.render_view();
        thread::sleep(Duration::from_millis(1000));
    }

    // thread::spawn(move || loop {
    //     let mut view = view_arc_clone.lock().unwrap();
    //     view.render_view();
    //     thread::sleep(Duration::from_millis(1000));
    // });
}
