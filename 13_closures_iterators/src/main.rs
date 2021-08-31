pub mod workout;
pub mod iterator;
pub mod shoes;

fn main() {
    iterator::do_thing();
    workout::workout();
}

fn move_closure() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    // this line will not compile because x has been moved
    // println!("Can't use x here: {:?}", x);

    let y = vec![1,2,3];
    assert!(equal_to_x(y));
}