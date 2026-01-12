fn main() {
    // v has been initialized with RWO perms
    let mut v: vec<i32> = vec![1, 2, 3];

    // v has been borrowed by num. which means:
    // 1) borrow removes WO perms from v.
    // 2) num gains RO perms. no W since it is not mutable
    // 3) the place *num gains the R permission. just read the value pointed to!
    let num: &i32 = &v[2];

    // after println! num is no longer in use, so compiler returns ownership to v (RWO)
    // num and num* lose all permissions
    println!("Third element is {}", *num);

    // after this execution, v is no longer use, and it loses all permissions (deallocation)
    v.push(4);

    // why both num and *num? Accessing data through a reference is not the same as manipulating the
    // reference itself. 
    // For example:
    let x = 0;

    let mut x_ref = &x;

    // x_ref has W permission, but *x_ref does not! See why? For this simple reason, they need
    // to be considered separately. We can assign a difference memory reference, but not mutate
    // whatever underlying data it points to.

    // we say permissions are defined on PLACES, and not just variables. This includes
    // variables (a), dereferences (*a), array accesses (a[0]), fields of places, (a.0 for
    // tuples, a.field for structs), or a combination of them ( *((*a)[0].1 ).
    
    // furthermore, places use permissions when they become unused because some permissions are
    // mutually exclusive.
    let mut v: Vec<i32> = vec![1, 2, 3];

    let num: &i32 = &v[2];

    println!("Third element is {}", *num);
    println!("Again, the third element is {}", *num);

    v.push(4);

    // when num is in use, v cannot be mutated or dropped. But that doesn't mean it is invalid
    // to use num AGAIN. in the snippet above, num is used twice, and perms aren't dropped until the compiler is sure it won't be used anymore. So when these perms are dropped, v's perms are restored. Ensure mutual
    // exclusivity!
    
    // using num again after mutating v is where things get hairy. This is the borrow checker.
    // See borrow_checker for relevant examples.
}

