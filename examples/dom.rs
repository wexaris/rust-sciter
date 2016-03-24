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


		if let Ok(Some(h1)) = body.find_first("body > h1") {
			println!("h1 {:?}", h1);

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

		let mut all = body.find_all("div > p").unwrap().expect("must be at least one div > p");
		assert!(all.is_empty() == false);
		assert_eq!(all.len(), 1);

		all.clear();

		if let Ok(Some(mut body)) = root.find_first("html > body") {

			println!("creating some elments");

			// DOM manipulation.
			// After creating the new Element, we can set only attributes for it until we'll attach it to the DOM.
			//
			let mut div = Element::create_at("div", &mut body).unwrap();
			div.set_style_attribute("outline", "1px solid orange");
			div.set_style_attribute("width", "max-content");
			div.set_style_attribute("padding", "5dip");

			let mut lb = Element::with_text("label", "Output: ");
			div.append(&lb).expect("wtf?");	// push as reference, we can access this `lb` still.

			let mut date = Element::with_type("input", "date");
			date.set_attribute("id", "mydate");
			date.set_attribute("value", "now");

			lb.append(&date).expect("wtf?");

			date.set_style_attribute("width", "100px");
			date.set_style_attribute("outline", "1px dotted gray");
			date.set_style_attribute("margin", "10px");


			lb.set_attribute("accesskey", "o");
			lb.set_style_attribute("color", "lightblue");
			lb.set_style_attribute("vertical-align", "middle");

			let mut progress = Element::create("progress");
			progress.set_attribute("max", "100");
			progress.set_attribute("name", "progress");

			div.append(&progress).expect("wtf?");

			// after attaching Element to DOM, we can set its styles, text, html or value.
			progress.set_value(Value::from(42));
		}



	}

}

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
