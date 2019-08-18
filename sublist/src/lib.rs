use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    match first_list.len().cmp(&second_list.len()) {
        Ordering::Less => {
            if is_sublist(first_list, second_list) {
                Comparison::Sublist
            } else {
                Comparison::Unequal
            }
        }
        Ordering::Greater => {
            if is_sublist(second_list, first_list) {
                Comparison::Superlist
            } else {
                Comparison::Unequal
            }
        }
        Ordering::Equal => {
            if first_list == second_list {
                Comparison::Equal
            } else {
                Comparison::Unequal
            }
        }
    }
}

fn is_sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> bool {
    first_list.is_empty()
        || second_list
            .windows(first_list.len())
            .any(|shorter_second_list| first_list == shorter_second_list)
}
