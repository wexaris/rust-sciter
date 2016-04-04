//! Sciter interop with native code and vice versa.

#![allow(unused_variables)]
#![allow(non_snake_case)]

#[macro_use]
extern crate sciter;

use sciter::{HELEMENT, Element, Value};

struct EventHandler {
	root: Option<Element>,
}

impl Drop for EventHandler {
	fn drop(&mut self) {
		println!("interop::EventHandler: Bye bye, HTML!");
	}
}

impl EventHandler {

	fn script_call_test(&self, args: &[Value], root: Element) -> Option<Value> {

		println!("root: {:?}", root);
		// return None;

		println!("calling 'hello'");
		let answer = root.call_function("hello", &make_args!("hello, rust!"));
		println!(" answer {:?}", answer);

		println!("get and call 'hello'");
		let answer = root.eval_script(r"hello");
		if answer.is_err() {
			return None;
		}
		let obj = answer.unwrap();
		let answer = obj.call(None, &make_args!("argument"), None);
		println!(" answer {:?}", answer);

		println!("eval 'hello'");
		let answer = root.eval_script(r#"hello("42");"#);
		println!(" answer {:?}", answer);

		println!("calling 'raise_error'");
		let answer = root.call_function("raise_error", &make_args!(17, "42", false));
		println!(" answer {:?}", answer);

		println!("calling unexisting function");
		let answer = root.call_function("raise_error2", &[]);
		println!(" answer {:?}", answer);

		return Some(Value::from(true));
	}

	fn NativeCall(&mut self, arg: String) -> Value {
		return Value::from(format!("Rust window ({})", arg));
	}

	fn GetNativeApi(&mut self, ) -> Value {

		fn on_add(args: &[Value]) -> Value {
			let ints = args.iter().map(|ref x| x.to_int().unwrap());
			// let sum: i32 = ints.sum();	// error: issue #27739
			let sum: i32 = ints.fold(0, |sum, x| sum + x);
			return Value::from(sum);
		}

		fn on_sub(args: &[Value]) -> Value {
			if args.len() != 2 || args.iter().any(|x| !x.is_int()) {
				return Value::error("sub requires 2 integer arguments!");
			}
			let ints: Vec<_> = args.iter().map(|ref x| x.to_int().unwrap()).collect();
			let (a,b) = (ints[0], ints[1]);
			return Value::from(a - b);
		}

		let on_mul = |args: &[Value]|  -> Value {
			let prod = args.iter().map(|ref x| x.to_int().unwrap()).fold(1, |total, x| total * x);
			Value::from(prod)
		};

		let mut api = Value::new();

		api.set_item(Value::from("add"), Value::from(on_add));
		api.set_item(Value::from("sub"), Value::from(on_sub));
		api.set_item(Value::from("mul"), Value::from(on_mul));

		println!("returning {:?}", api);
		return api;
	}

	fn calc_sum(&mut self, a: i32, b: i32) -> i32 {
		a + b
	}

}


impl sciter::EventHandler for EventHandler {

	fn attached(&mut self, root: HELEMENT) {
		self.root = Some(Element::from(root));
	}

	dispatch_script_call! {

		fn NativeCall(String);

		fn GetNativeApi();

		fn calc_sum(i32, i32);
	}

	fn on_script_call(&mut self, root: HELEMENT, name: &str, argv: &[Value]) -> Option<Value> {

		let args = argv.iter().map(|ref x| format!("{}", &x)).collect::<Vec<String>>().join(", ");
		println!("script->native: {}({}), root {:?}", name, args, root);

		let handled = self.dispatch_script_call(root, name, argv);
		if handled.is_some() {
			return handled;
		}

		match name {

			"ScriptCallTest" => {
				return self.script_call_test(argv, Element::from(root));
			},

			_ => (),
		}

		return None;
	}

}

fn main() {
	let handler = EventHandler { root: None };
	let mut frame = sciter::Window::new();
	frame.event_handler(handler);
	frame.load_file("interop.htm");
	frame.run_app(true);
}
