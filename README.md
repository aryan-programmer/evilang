# evilang

A simple interpreted programming language with a syntax similar to JavaScript developed in Rust. The name is derived from the adage "eval is evil".

## Supported Features

- Logical & Arithmetic Operators
- if, else if, else ladder
- for, while and do..while loops
- Block Scoping
- Functions
- Function Closures
- Classes
- Garbage Collection for Runtime Objects (using the `gc` crate)
- Namespaces & Modules
- Vectors
- Native Class Bindings

### Native Class Bindings

The `Vector` class in Evilang is an example of a native class with its constructor & member functions written in Rust, but instantiated & used in Evilang. See ./evilang_lib/src/lib/interpreter/environment/native_items/classes/vector.rs

Native class bindings are implemented with the help of a custom `syn`-based procedural macro.

## How to run

CMD/Bash:

```
cargo run -- --file <FILE>
```

Powershell:

```
cargo run `-- --file <FILE>
```

## Code Samples

### For Loop

See ./resources/for_loop.evil

```
for({
		let i = 1;
		let j = 2;
		i += j;
	};
	i <= 10;
	{
		i += 1;
		j *= 2;
	}){
	println(i,",",j);
}
```

Output:

```
3,2
4,4
5,8
6,16
7,32
8,64
9,128
10,256
```

### Functions, Function Closures and if...else

See ./resources/functions.evil

```
fn useState(default) {
	let value = default;
	fn returner(name) {
		if(name == "get") {
			return get;
		} else if(name == "set") {
			return set;
		}
		fn get() {
			return value;
		}
		fn set(nval) {
			value = nval;
		}
	}
	return returner;
}
let state = useState(10);
let getX = state("get");
let setX = state("set");

println(getX());
setX(getX() + 32);
println(getX());
setX(-13);
println(getX());
```

Output:

```
10
42
-13
```

### Vectors

See ./resources/tests/vector_test/main.evil

```
import "../common/index.evil" as tests_common;
let assert = tests_common.assert;

let ran_basic_test = false;

fn basic_test() {
	let v = new Vector();
	// len()
	assert(v.len() == 0, "Expected vector to be empty");

	// push()
	v.push(42);
	v.push("string");
	v.push(-3);
	assert(v.len() == 3, "Expected vector have size 3");
	assert(v.get(0) == 42, "Expected vector's 0th element to be 42");
	assert(v.get(1) == "string", "Expected vector's 1st element to be \"string\"");
	assert(v.get(2) == -3, "Expected vector's 2nd element to be -3");

	// insert()
	v.insert(1, "value");
	assert(v.equals(Vector::from(42, "value", "string", -3)), "Expected vectors to match after insert");
	assert(v.len() == 4, "Expected vector have size 4 after insert");
	assert(v.get(0) == 42, "Expected vector's 0th element to be 42");
	assert(v.get(1) == "value", "Expected vector's 1st element to be \"value\"");
	assert(v.get(2) == "string", "Expected vector's 2nd element to be \"string\"");
	assert(v.get(3) == -3, "Expected vector's 3rd element to be -3");

	// set()
	v.set(0, -v.get(3));
	v.set(3, -42);
	assert(v.equals(Vector::from(3, "value", "string", -42)), "Expected vectors to match after insert");
	assert(v.get(0) == 3, "Expected vector's 0th element to be set to 3");
	assert(v.get(3) == -42, "Expected vector's 3rd element to be set to -42");

	// pop()
	let popped_val =  v.pop();
	assert(v.equals(Vector::from(3, "value", "string")), "Expected vectors to match after pop");
	assert(popped_val == -42, "Expected the popped value to be -42");

	// remove()
	let removed_val =  v.remove(1);
	assert(v.equals(Vector::from(3, "string")), "Expected vectors to match after pop");
	assert(removed_val == "value", "Expected the removed value to be \"value\"");

	ran_basic_test = true;
}

fn medium_tests() {
	let v = Vector::from(42, "value", "string", -3);

	// resize()
	v.resize(3, null);
	assert(v.equals(Vector::from(42, "value", "string")), "Expected vectors to match after resize(3)");

	// clone()
	let v_snap = v.clone();
	assert(v.equals(v_snap), "Expected vector's to be equal to it's clone");
	assert(v.len() == v_snap.len(), "Expected vector's length to be equal to it's clone's length");

	// resize() again
	v.resize(5, 0);
	assert(v.equals(Vector::from(42, "value", "string", 0, 0)), "Expected vectors to match after resize(5, 0)");
	assert(v.len() == 5, "Expected vector have size 5 after resize");

	// reserve()
	v.reserve(10);
	assert(v.equals(Vector::from(42, "value", "string", 0, 0)), "Expected vectors to match after reserve");
	assert(v.len() == 5, "Expected vector's size to still be 5 after reserve");
	assert(v.capacity() > 10, "Expected vector's capacity to be 10 (or higher) after reserve");

	// clear()
	v.clear();
	assert(v.equals(Vector::from()), "Expected vectors to be empty after clear (1)");
	assert(v.equals(new Vector()), "Expected vectors to be empty after clear (2)");
	assert(v.len() == 0, "Expected vector's size to be 0 after clear");
}

fn functional_tests(){
	let v = Vector::from_exec_n(5, fn _(i){return i + 1;});
	assert(v.equals(Vector::from(1, 2, 3, 4, 5)), "from_exec_n");
	v.for_each(fn _(v, i){
		assert(v - 1 == i, "Expected items to match");
	});
	let v_strs = v.map(to_string);
	assert(v_strs.equals(Vector::from("1", "2", "3", "4", "5")), "map(to_string)");

	v.map_inline(fn _(v, i){
		return v * i;
	});
	assert(v.equals(Vector::from(0, 2, 6, 12, 20)), "map_inline");

	let reduce = v.reduce(0, fn _(p, v){return p + v;});
	assert(reduce == 40, "reduce");

	let reduce_strs = v.reduce("", fn _(p, v, i){
		if (i != 0) p += "
";
		p += "(" + to_string(i) + ") = " + to_string(v);
		return p;
	});
	assert(reduce_strs == "(0) = 0
(1) = 2
(2) = 6
(3) = 12
(4) = 20", "reduce with strings");
}

let tests = Vector::from(basic_test, medium_tests, functional_tests);
tests.for_each(fn iter(v){
	println("Running test ", v);
	v();
	println("Test completed");
});

assert(ran_basic_test, "Basic test was not executed");
```

Output:

```
Running test basic_test
Test completed
Running test medium_tests
Test completed
Running test functional_tests
Test completed
```

## Status & Rationale

This is not a fully fledged language, it was created as a side-project to learn how to develop a parser & interpreter and learn the intricacies of Rust.
