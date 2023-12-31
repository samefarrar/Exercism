#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        return Comparison::Equal
    }
    if _first_list.len() > _second_list.len() {
        if is_sublist(_second_list, _first_list) {
            return Comparison::Superlist
        } else {
            return Comparison::Unequal
        }
    } else if _second_list.len() > _first_list.len() {
        if is_sublist(_first_list, _second_list) {
            return Comparison::Sublist
        } else {
            return Comparison::Unequal
        }
    } else {
        return Comparison::Unequal
    }
}

fn is_sublist<T: PartialEq>(shorter_list: &[T], longer_list: &[T]) -> bool {
    if shorter_list.is_empty() {
        return true;
    }
    for i in 0..=longer_list.len() - shorter_list.len() {
        if longer_list[i..i + shorter_list.len()] == shorter_list[..] {
            return true;
        }
    }
    false
}
