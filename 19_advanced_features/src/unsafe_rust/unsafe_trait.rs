unsafe trait Foo {

}

unsafe impl Foo for i32 {

}

// we can use this to implement a trait on a type when --
// okay, honestly I didn't really understand when this
// can be used and when it can't. We'll have to cross
// that bridge when we get to it. The only thing I got
// was that we can apply a trait to something like a
// raw pointer where there are things that definitely
// can't be checked by the compiler.