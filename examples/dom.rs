#![allow(unused_variables)]

extern crate sciter;

use sciter::Element;
use sciter::Value;

struct Handler;

impl sciter::EventHandler for Handler {

	fn document_complete(&self, root: sciter::HELEMENT) {

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


		if let Some(h1) = body.find_first("body > h1") {
			println!("{:?}", h1);

			let h1_parent = h1.parent().expect("come on!");
			assert_eq!(h1_parent, body);

			let text = h1.get_text();
			assert_eq!(text, "Herman Melville - Moby-Dick");

			let html = h1.get_html(true);
			assert_eq!(html.as_slice(), r"<h1>Herman Melville - Moby-Dick</h1>".as_bytes());

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

		let mut all = body.find_all("div > p").unwrap();
		assert!(all.is_empty() == false);
		assert_eq!(all.len(), 1);

		all.clear();

		if let Some(mut body) = root.find_first("html > body") {

			println!("creating some elments");

			// DOM manipulation.
			// After creating the new Element, we can set only attributes for it until we'll attach it to the DOM.
			//
			let mut div = Element::create("div");

			let mut el = Element::create("output");
			el.set_attribute("type", "date");
			el.set_attribute("id", "mydate");

			let mut lb = Element::with_text("label", "Output: ");

			body.append(&div).expect("wtf?");
			div.append(&lb).expect("wtf?");	// push as reference, we can access this `lb` still.

			lb.push(el);			// push like `Vec.push()` - i.e. forgot about `el`.

			lb.set_attribute("accesskey", "o");
			lb.set_style_attribute("color", "lightblue");

			let mut progress = Element::create("progress");
			progress.set_attribute("max", "100");
			progress.set_attribute("name", "progress");

			div.append(&progress).expect("wtf?");

			// after attaching Element to DOM, we can set its styles, text, html or value.
			progress.set_value(Value::from(42));
		}



	}

}

impl sciter::event::WindowEventHandler for Handler {}

fn testing_dom() {
	let mut frame = sciter::Window::new();
	frame.event_handler(Handler);
	frame.set_title("DOM sample");
	frame.load_file("http://httpbin.org/html");
	frame.run_app(true);
}

fn main() {
	testing_dom();
}
