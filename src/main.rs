fn main() {
    let mut data = [5, 2, 4, 6, 1, 3];
    sortable::insertion_sort(&mut data);
    println!("{:?}", data);
}


mod sortable {
    pub(crate) fn insertion_sort<T: Ord>(data: &mut [T]) {
        for i in 1..data.len() {
            let mut j = i;
            while j > 0 && data[j - 1] > data[j] {
                data.swap(j - 1, j);
                j -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort() {
        let mut data = [5, 2, 4, 6, 1, 3];
        sortable::insertion_sort(&mut data);
        assert_eq!(data, [1, 2, 3, 4, 5, 6]);
    }
}