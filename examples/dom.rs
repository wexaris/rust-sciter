#![allow(unused_variables)]
#![allow(unused_must_use)]

extern crate sciter;

use sciter::{Value, Element, HELEMENT};
use sciter::dom::event::*;

#[derive(Default)]
struct Handler {
	progress: Option<Element>,
	state: bool,
}

impl sciter::EventHandler for Handler {

	fn get_subscription(&mut self) -> Option<EVENT_GROUPS> {
		Some(default_events() | EVENT_GROUPS::HANDLE_TIMER)
	}

	fn attached(&mut self, root: sciter::HELEMENT) {
		let root = Element::from(root);
		println!("attached to {}", root);
		if root.test("progress") {
			self.progress = Some(root.clone());
			self.state = false;
		}
	}

	fn detached(&mut self, root: sciter::HELEMENT) {
		let root = Element::from(root);
		println!("detaching from {}", root);
	}

	fn on_event(&mut self, root: HELEMENT, source: HELEMENT, target: HELEMENT, code: BEHAVIOR_EVENTS, phase: PHASE_MASK, reason: EventReason) -> bool {
		if phase != PHASE_MASK::BUBBLING {
			return false;
		}

		match code {
			BEHAVIOR_EVENTS::BUTTON_CLICK => {

				let source = Element::from(source);
				let mut target = Element::from(target);

				println!("button click on target {}", target);

				if self.progress.is_some() && *self.progress.as_ref().unwrap() == target {
					self.state = !self.state;

					if self.state {
						println!("starting timer");
						target.set_value(Value::from(0));
						target.start_timer(1000, 1).ok();
					} else {
						println!("stopping timer");
						target.stop_timer(1);

						let cur = target.get_value();
						target.set_attribute("title", &format!("Current value is {}. Click to start timer again.", cur));
					}
				}

				true
			}
			_ => false
		}
	}

	fn on_timer(&mut self, root: HELEMENT, timer_id: u64) -> bool {
		println!("timer {} tick on {}", timer_id, Element::from(root));
		if timer_id == 1 && self.progress.is_some() {
			let e = self.progress.as_mut().unwrap();
			let max_attr = e.get_attribute("max").unwrap();
			let max: f64 = max_attr.parse().unwrap();

			let v = e.get_value();
			let next = v.to_float().unwrap() + 5.0;

			if next > max {
				println!("that's enough, finish.");
				self.state = false;
				e.stop_timer(1);
			}

			e.set_value(Value::from(next));
			e.set_attribute("title", &format!("Current value is {}. Click to stop timer, if need.", next));

			return true;
		}
		false
	}

	fn document_complete(&mut self, root: sciter::HELEMENT, source: sciter::HELEMENT) {

		let root = Element::from(root);
		assert_eq!(root.get_tag(), "html");
		println!("root {:?}", root);

		let head = root.first_child().expect("empty <html>?");
		assert_eq!(head.get_tag(), "head");
		assert_eq!(head.index(), 0);
		println!("head {:?}", head);

		let body = head.next_sibling().expect("only <head>?");
		assert_eq!(body.get_tag(), "body");
		assert_eq!(body.index(), 1);
		println!("body {:?}", body);

		assert_eq!(body.first_sibling().expect("must be head"), head);
		assert_eq!(body.last_sibling().expect("must be body"), body);

		println!("for loop in children");
		for e in root.children() {
			println!("child {:?}", e);
		}

		println!("for loop in ref");
		for e in &root {
			println!("child {:?}", e);
		}

		if let Ok(Some(h1)) = body.find_first("body > h1") {
			println!("h1 {:?}", h1);

			let h1_parent = h1.parent().expect("come on!");
			assert_eq!(h1_parent, body);

			let text = h1.get_text();
			assert_eq!(text, "Herman Melville - Moby-Dick");

			let html = h1.get_html(true);
			assert_eq!(html.as_slice(), br"<h1>Herman Melville - Moby-Dick</h1>".as_ref());

			let value = h1.get_value();
			assert!(value.is_string());
			assert_eq!(value.as_string().unwrap(), text);
		}

		if let Some(mut h1) = body.first_child() {
			println!("changing h1 attributes");
			h1.set_style_attribute("color", "lightblue");
			h1.set_style_attribute("outline", "1px solid orange");
			h1.set_attribute("title", "yellow!");
		}

		let mut all = body.find_all("div > p").unwrap().expect("must be at least one div > p");
		assert!(!all.is_empty());
		assert_eq!(all.len(), 1);

		all.clear();

		if let Ok(Some(mut body)) = root.find_first("html > body") {

			println!("creating some elments");

			// DOM manipulation.
			// After creating the new Element, we can set only attributes for it until we'll attach it to the DOM.
			//
			let mut div = Element::with_parent("div", &mut body).unwrap();
			div.set_style_attribute("outline", "1px solid orange");
			div.set_style_attribute("width", "max-content");
			div.set_style_attribute("padding", "5dip");

			let mut lb = Element::with_text("label", "Output: ").unwrap();
			div.append(&lb).expect("wtf?");	// push as reference, we can access this `lb` still.

			let mut date = Element::with_type("input", "date").unwrap();
			date.set_attribute("id", "mydate");
			date.set_attribute("value", "now");

			lb.append(&date).expect("wtf?");

			date.set_style_attribute("width", "100px");
			date.set_style_attribute("outline", "1px dotted gray");
			date.set_style_attribute("margin", "10px");


			lb.set_attribute("accesskey", "o");
			lb.set_style_attribute("color", "lightblue");
			lb.set_style_attribute("vertical-align", "middle");

			let mut progress = Element::create("progress").unwrap();
			progress.set_attribute("max", "100");
			progress.set_attribute("id", "id1");
			progress.set_attribute("title", "Click to start timer.");

			div.append(&progress).expect("wtf?");

			// after attaching Element to DOM, we can set its styles, text, html or value.
			progress.set_value(Value::from(42));
			progress.set_style_attribute("behavior", "progress clickable");

			// attach custom handler to this element
			// since timers are not sinking/bubbling, we need to attach our handler to the target element directly.
			let handler = Handler::default();
			progress.attach_handler(handler).expect("can't attach?");

			let mut e = Element::with_text("span", " <-- check tooltip").unwrap();
			div.append(&e);

			e.set_style_attribute("font-style", "italic");
		}
	}
}

fn main() {
  let mut frame = sciter::WindowBuilder::main_window()
  	.with_size((750, 950))
  	.create();
	frame.event_handler(Handler::default());
	frame.set_title("DOM sample");
	frame.load_file("http://httpbin.org/html");
	frame.run_app();
}
