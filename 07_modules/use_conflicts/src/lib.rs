use std::fmt;
use std::io;

fn function1() -> fmt::Result {

}

fn function2() -> io::Result<()> {

}

// if we'd done e.g. use std::fmt::Result, there
// would have been a collision

// another solution is the "as" keyword to rename an import
use std::io::Result as IoResult;

fn function3() -> IoResult<()> {

}

// we can use nested paths to avoid repeating
use std::{cmp::Ordering, vec};

// if one of the nested paths is the root, this works
use std::io::{ self, Write };

// you can bring in all public items using the glob operator
use std::collections::*;