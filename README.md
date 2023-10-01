# show-bytes

Display bytes as printable ascii with escape sequences as needed.

## Examples

```rust
use show_bytes::println;

// byte slice
let bytes_slice: &[u8] = &[72, 101, 108, 108, 111, 0, 255];
println(bytes_slice);

// byte vector
let bytes_vec: Vec<u8> = vec![72, 101, 108, 108, 111, 0, 255];
println(&bytes_vec);
println(bytes_vec);

// byte array
let bytes_array: [u8; 7] = [72, 101, 108, 108, 111, 0, 255];
println(bytes_array);
println(&bytes_array);

// byte iterator
let mut bytes_iter = [72, 101, 108, 108, 111, 0, 255].iter();
println(bytes_iter.clone());
println(&mut bytes_iter);

// &str
let bytes_str: &str = "hello\0\x7f";
println(bytes_str.bytes());
let bytes_str = &bytes_str;
println(bytes_str.bytes());

// String
let bytes_string: String = bytes_str.to_string();
println(bytes_string.bytes());
let bytes_string = &bytes_string;
println(bytes_string.bytes());

// OsString
let bytes_os_string: OsString = OsString::from(bytes_str);
println(bytes_os_string.as_bytes());
let bytes_os_string: &OsString = &bytes_os_string;
println(bytes_os_string.as_bytes());

// OsStr
let bytes_os_str: &OsStr = OsStr::from_bytes(bytes_slice);
println(bytes_os_str.as_bytes());

// Box<[u8]>
let boxed_slice: Box<[u8]> = Box::new([72, 101, 108, 108, 111, 0, 255]);
println(boxed_slice.iter());
println(&mut boxed_slice.iter());

// std::io::Cursor<Vec<u8>>
let cursor = Cursor::new(vec![72, 101, 108, 108, 111, 0, 255]);
let bytes_from_cursor: Vec<u8> = cursor.into_inner();
println(&bytes_from_cursor);
println(bytes_from_cursor);

// std::collections::VecDeque<u8>
let mut vec_deque = VecDeque::new();
vec_deque.push_back(72);
vec_deque.push_back(101);
vec_deque.push_back(108);
vec_deque.push_back(108);
vec_deque.push_back(111);
vec_deque.push_back(0);
vec_deque.push_back(255);
println(&vec_deque);
println(vec_deque);

// Cow<[u8]>
let cow_slice: Cow<[u8]> = Cow::Borrowed(&[72, 101, 108, 108, 111, 0, 255]);
println(cow_slice.iter());
let cow_slice: Cow<[u8]> = Cow::Owned(vec![72, 101, 108, 108, 111, 0, 255]);
println(cow_slice.iter());

// Arc<Vec<u8>>
let arc_slice = Arc::new(vec![72, 101, 108, 108, 111, 0, 255]);
println(arc_slice.iter());

// Rc<Vec<u8>>
let rc_slice = Rc::new(vec![72, 101, 108, 108, 111, 0, 255]);
println(rc_slice.iter());
```

## Links

- [Documentation](https://docs.rs/show-bytes/)
