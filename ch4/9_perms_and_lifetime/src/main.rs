fn main() {
    let mut x = 1;

    // lifetime of y starts here
    let y = &x;

    // lifetime of y ends here
    // W perm is returned to x at this point, allowing for L11
    let z = *y;

    x += z;
}
