// dynamically sized type (DST) (or unsized type)
// example is type str (not &str, since we don't known length until runtime)
// &str actually is two values; the the address and the length

// the "golden rule" of DSTs is that types with unknown sizes must always be
// behind a pointer of some kind (e.g. &str)

// we can hide them between different types of pointers, so Box<str> or Rc<str>
// also works. Traits are actually the same (although I'm not sure why traits
// are dynamically sized)

// rust has a trait called Sized that is automatically implemented for any type
// whose size is known at compile time; in addition, all generics are automatically
// bound to types that are sized (e.g. generic<T: Sized>). This can be relaxed
// with e.g. generic<T: ?Sized> (which means may or may not be and is only available
// with Sized and no other trait). Also note in the case of:
fn generic<T: ?Sized>(t: &T) {

}

// the parameter t is forced to be a behind a pointer, since it might not be sized