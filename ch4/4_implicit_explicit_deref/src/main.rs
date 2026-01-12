fn main() {
    let x: Box<i32> = Box::new(-1);
    let x_abs1 = i32::abs(*x);      // explicit dereference
    let x_abs2 = x.abs();           // implicit dereference
    assert_eq!(x_abs1, x_abs2);

    let r: &Box<i32> = &x;
    let r_abs1 = i32::abs(**r);  // explicit dereference (twice)
    let r_abs2 = r.abs();        // implicit dereference (twice)
    assert_eq!(r_abs1, r_abs2);

    // opposite direction: passes in the pointer to the string, not the value. expected
    // argument for str::len
    let s = String::from("Hello");
    let s_len1 = str::len(&s); // explicit reference
    let s_len2 = s.len();      // implicit reference
    assert_eq!(s_len1, s_len2);
}

// the dot operator is "syntactic sugar" to call these functions!
// makes sure you don't need to directly pass in the value; happens implicitly with deref.
// // note that with `r`, we illustrate it can dereference multiple layers!
