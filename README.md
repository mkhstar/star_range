
# star_range

  Rust implementation of python's range function.

### Motive

Rust has got a basic ``..`` iterator syntax but it is difficult to use if you want a step in the iteration process. One will need to implement it inside a loop with series of checks to get the right values. ``star_range::range`` takes care of everything.

### Parameters

|parameter_name|type|description
|--|--|--|
| start | isize  or  f64 | The start of the iterator
| end | isize  or  f64 | The end of the iterator
| step | isize  or  f64 | The step of the iterator, Supports negative values


### Returns 

``StarRange<T>`` struct that implements Iterator

### Usage 


Since it is not yet published to cargo, you can download the lib.rs file to you src directory and rename it to star_range.rs

```rust
mod star_range;
use star_range::range;

for x in range(1, 10, 2) {
	println!("{}", x); 
	// Prints
	/*
	1
	3
	5
	7
	9
	*/
}
for x in range(10, 1, -2) {
	println!("{}", x); 
	// Prints
	/*
	10
    	8
    	6
    	4
    	2
	*/
}
for x in range(1.0, 10.0, 1.5) {
	println!("{}", x); 
	// Prints
	/*
	1
    	2.5
    	4
    	5.5
    	7
    	8.5
	*/
}


```


### Pull Requests

  

Pull requests are welcome

  

### License

  

Star Range may be freely distributed under the MIT license.
