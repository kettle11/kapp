use kettlewin::*;
fn main() {
    let (app, mut event_loop) = initialize();
    event_loop.run_async(app, run);
}

async fn run(app: Application, mut events: Events) {
    let mut _window = app.new_window().build().unwrap();
    
    // Loop forever!
    loop {
        match events.next_event().await {
            Event::WindowCloseRequested { .. } => app.quit(),
            Event::Draw { .. } => {}
            _ => {}
        }
    }
}
