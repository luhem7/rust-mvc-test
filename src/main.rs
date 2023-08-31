use std::{
    thread,
    sync::{Arc,Mutex},
    time::Duration,
    io
};

use clearscreen::clear;

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

struct View {
    last_user_input: String
}

impl View {
    fn new() -> Self {
        Self {
            last_user_input: "".to_string()
        }
    }

    fn render_view(&self, model: &mut Model) {
        clear().expect("failed to clear screen");
        println!("Rendering View {}", model.counter);
        println!("Last user input: {}", self.last_user_input);
    }
}

fn main() {
    println!("Initialized App.");

    let view_arc = Arc::new(Mutex::new(View::new()));
    let view_arc_clone = Arc::clone(&view_arc);

    let model_arc = Arc::new(Mutex::new(Model::new()));
    let model_arc_clone = Arc::clone(&model_arc);

    // Rendering the view
    thread::spawn(move || loop {
        let view = view_arc_clone.lock().unwrap();
        let mut model = model_arc_clone.lock().unwrap();
        model.increment_counter();
        view.render_view(&mut model);
        thread::sleep(Duration::from_millis(1000));
    });

    // Handling user input
    let view_arc_clone = Arc::clone(&view_arc);
    loop {
        thread::sleep(Duration::from_millis(500));
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if input.is_empty() { continue; }
        else if input == "q" { break; }
        else {
            let mut view = view_arc_clone.lock().unwrap();
            view.last_user_input = input; 
        }
    }
}
