pub fn insert_sort<T>(values: &mut [T])
    where T: Ord
{
    // Generic function for lists with types that can be ordered
    // Standard insert_sort algorithm with constant memory and O(n^2) time
    for i in 0..values.len() {
        for j in (0..i).rev() {
            if values[j] >= values[j+1] {
                values.swap(j, j+1);
            }
            else {
                break
            }
        }
    }
}

#[test]
fn test_insert_sort_zero(){
    // Tests sorting list of length 0
    let mut values: [i32; 0] = [];
    insert_sort(&mut values);
    assert_eq!(values, []);
}

#[test]
fn test_insert_sort_one() {
    // Tests sort list of length 1
    let values = [1];
    insert_sort(&mut values);
    assert_eq!(values, [1]);
}

#[test]
fn test_insert_sort_many() {
    // Tests sort list of length n
    let mut values = vec![5, 2, 3, 1, 6, 4];
    insert_sort(&mut values);
    assert_eq!(values, vec![1, 2, 3, 4, 5, 6]);
}
