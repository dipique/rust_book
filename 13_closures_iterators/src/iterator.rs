mod counter;
use counter::Counter;

pub fn do_thing() {
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    let sum: i32 = v1_iter.sum();
    println!("Sum is {}", sum);

    // this line will not compile because sum() consumes the iterator
    // println!("Sum is {} for iterator {:?}", sum, v1_iter);

    iterators_are_lazy();
}

fn iterators_are_lazy() {
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    let mapped = v1_iter.map(|i| i + 1);

    // at this time, no operation has been performed (just like .Select in C#);
    // we need to use a consuming function to execute the operation/query

    let data: Vec<i32> = mapped.collect();
    println!("Now the operation has been executed: {:?}", data);
}

#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new(8)
        .zip(Counter::new(8).skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(90, sum);
}