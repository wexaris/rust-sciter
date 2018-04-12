#[macro_use]
extern crate sciter;
use sciter::Value;

struct EventHandler;

impl EventHandler {
	// script handler
	fn exec_task(&self, task_no: i32, progress: sciter::Value, done: sciter::Value) -> bool {

		use std::{thread, time};

		// make values thread-safe
		let progress = progress.isolate();
		let done = done.isolate();

		thread::spawn(move || {
			// and use them here:
			for i in 1..100 {
				// call `onProgress` callback
				thread::sleep(time::Duration::from_millis(100));
				progress.call(None, &make_args!(i), Some(file!())).unwrap();
			}

			// call `onDone` callback
			done.call(None, &make_args!(task_no), Some(file!())).unwrap();
		});
		true
	}
}

impl sciter::EventHandler for EventHandler {
	// route script calls to our handler
	dispatch_script_call! {
		fn exec_task(i32, Value, Value);
	}
}

fn main() {
	let html = include_bytes!("threads.htm");
  let mut frame = sciter::WindowBuilder::main_window()
  	.with_size((1200, 900))
  	.create();
	frame.event_handler(EventHandler);
	frame.load_html(html, None);
	frame.run_app();
}
