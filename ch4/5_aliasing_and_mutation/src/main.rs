fn main() {
    // v allocates a heap array of a certain capacity.
    
    /* an internal cap variable is 3. when `push` is called, the vector creates a new
     allocation with larger capacity, copies the elements over, and deallocates the original
     array on the heap. `1 2 3 4` will likely (assignment is not NECESSARILY different, but
     the full "copy" still takes places regardless) be in a different memory location (but still in the
     heap segment of memory). */
    let mut v: Vec<i32> = vec![1, 2, 3];
    // dynamic allocation
    v.push(4);

    // bringing references back into the mix
    // -- UNDEFINED BEHAVIOR --
    // num points to invalid memory because of the way rust resizes vectors! It is not the same
    // location in memory!
    // the following code below will thus throw a compiler error. Reference, THEN reallocation
    // which changes memory.
    // compiler interprets this as a conflict of "borrows" to the memory.
    // folllows Pointer Safety Principle: data should never be aliased and mutated at the same
    // time. This is important because remember: REFERENCE ARE NON-OWNING POINTERS. This
    // conflict isn't an issue for allocations which need ownership, and will be deallocated
    // when a new owner is declared. 
    let num: &i32 = &v[2];
    v.push(5);
    println!("Third element is {}", *num);
}

// -- The Borrow Checker --

/* The borrow checker has three kinds of permissions on their data: Read (R), Write (W), and Own
 (O)
1) R - data can be copied to another location
2) W - data can be mutated
3) O - data can be moved or dropped (new ownership)

This is how the compiler thinks about your program before the program is executed. That is, it all happens before runtime ever happens.

A variable has RO permissions by default. It also gains W if annotated with `let mut`.

References can temporarily remove these permissions! See `remove_permissions`
