// Everything is an expression:
//fn defines a function
fn main() /* -> str */ {
	//https://fasterthanli.me/articles/a-half-hour-to-learn-rust
	// Variable bindings are immutable and cannot be reassigned
	let _i : i16=200; //i8 i16 default:[SIGNED-i32]
	// mut makes a variable binding mutable
	let mut _j:i16;
	_j=250;
	// Cannot access uninitialized variables in methods.
	
	//lack of name | special name | throw-away | do nothing
	let _ = 42;

	// Supports tuple
	let _pair =('a', 17);
	//pair.0;
	//pair.1

	// tuples can be destructured
	let (_l,_r) = ('a',18);
	// You can use _ to throwaway 'b'
	let (_, _right) = ('b','c');

	
	println!("Hello, world!");

	//Pair of brackets define a scope | IIFE (Anonymous funciton) in JS 
	{
		// Blocks need to evaluate a value. 
		// Last line of a block is called a tail. They return a value.
	}

	// . is used to access attributes or functions
	// :: is used for namespaces

	// use std::cmp:min

	// Type system:
	struct Blah{
		odd: bool,
		value: i16
	}

	impl Blah {
		fn is_positive(self) -> bool {
			self.value>0
		}
	}


	let hap = Blah{odd:true,value:125};

	fn inside_fn(n: Blah){
		match n.value {
			1 => println!("One"),
			2 => println!("Two {}",n.odd),
			_ => println!("{}", n.is_positive()), // _ is used as a catch all like default in switch:case
		}
	}
	inside_fn(hap);

	// Vec type means an array
	// macros can identified with "!" at the end
	/* e.g. println!("Hello") is a macro.
	println! expands to 
	use std::io:{self, Write};
	io::stdout()
		.lock()
		.write_all(b"Hello")
		.unwrap();
	 */

	 // throw exception --> panic!("This panics");

	 /*
	 Option<T> | Some<T>
	 
	 Option<T> is an enum with { None, Some(T)}
	 It supports the function .unwrap()

	 Result<T,E>{
		Ok(T),
		Err(E)
	 }

	 try catch to handle panic ==> .expect("custom error message")

	 Iterators | They are computed lazily on demand
	 let natural_numbers = 1..; // notation is called a range
	 */
	
}
