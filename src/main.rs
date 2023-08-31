use std::{
    thread,
    sync::{Arc,Mutex},
    time::Duration,
};

struct Model {
    counter: i32
}

impl Model {
    fn new() -> Self {
        Self {
            counter: 0
        }
    }

    fn increment_counter(&mut self) {
        self.counter += 1;
    }
}

struct View;

impl View {
    fn new() -> Self {
        Self
    }

    fn render_view(&self, model: &mut Model) {
        println!("Rendering View {}", model.counter);
    }
}

fn main() {
    println!("Initialized App.");

    let view_arc = Arc::new(Mutex::new(View::new()));
    let view_arc_clone = Arc::clone(&view_arc);

    let model_arc = Arc::new(Mutex::new(Model::new()));
    let model_arc_clone = Arc::clone(&model_arc);

    loop {
        let view = view_arc_clone.lock().unwrap();
        let mut model = model_arc_clone.lock().unwrap();
        model.increment_counter();
        view.render_view(&mut model);
        thread::sleep(Duration::from_millis(1000));
    }

    // thread::spawn(move || loop {
    //     let mut view = view_arc_clone.lock().unwrap();
    //     view.render_view();
    //     thread::sleep(Duration::from_millis(1000));
    // });
}
