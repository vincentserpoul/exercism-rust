#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    }

    if is_sublist(first_list, second_list) {
        return Comparison::Sublist;
    }

    if is_sublist(second_list, first_list) {
        return Comparison::Superlist;
    }

    Comparison::Unequal
}

fn is_sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> bool {
    if first_list.is_empty() {
        return true;
    }

    second_list
        .windows(first_list.len())
        .any(|shorter_second_list| first_list == shorter_second_list)
}
