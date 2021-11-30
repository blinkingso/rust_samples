/// Generic Data Types
fn main() {
    let list = vec![-10, 1, -100, 20, 3, 10];
    let largest = largest(&list);
    println!("largest is : {}", largest);
}

/// get largest from the list.
fn largest<T>(list: &[T]) -> T
where
    T: PartialOrd + Ord + Clone,
{
    if list.is_empty() {
        panic!("list is empty");
    }

    let mut largest = &list[0];
    for item in list {
        if *item > *largest {
            largest = item;
        }
    }

    largest.clone()
}

#[cfg(test)]
mod tests {

    #[test]
    fn generic_data_types() {}
}
