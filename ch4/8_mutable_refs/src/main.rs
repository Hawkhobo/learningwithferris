fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3];

    let num: &mut i32 = &mut v[2];

    *num += 1;
    println!("Third element is {}", *num);

    // this causes a compiler error, but it didn't before. This is because I inserted code
    // below, so num is still "in use". Compiler is smart and knows to drop num when the last
    // line containing it has been reached.
    //println!("Vector is now {:?}", v);

    // can also downgrade mutable references!
    // this one is immutable
    let num2: &i32 = & *num;

    // now num only has R perms; and num2 as well, since it's a shared reference. so multiple
    // borrows is fine. not ownership
    println!("{} {}", *num, *num2);
}

// this is a mutable reference (also known as a unique reference).
// So far we've only seen immutable ones (shared references)
// Two important differences in these permissions
// 1) v loses the R permission, which means it now has NO permissions
// 2) since num is mutable, *num has gained the W permisssion (mutable!)
// Step 1) still maintains independence in mutation and aliasing!
