fn main () {
    let s = String::from ("hello world");
    let a = find_first_word (&s);
    let b = find_first_word (&s[..4]);
    let c = "literal string";
    let d = find_first_word (&c[..3]);
    let e = find_first_word (c);
    // s.clear (); compile-time error : cannot clear future borrowed reference
    println! ("{} {} {} {} {}", a, b, c, d, e);

    // number slices
    let z = [1, 2, 3, 4, 5];
    let y = &z[..2];
    println! ("{:?}", y);
}

fn find_first_word (s : &str) -> &str {
    let bytes = s.as_bytes ();

    for (i, &c) in bytes.iter().enumerate() {
        if c == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

