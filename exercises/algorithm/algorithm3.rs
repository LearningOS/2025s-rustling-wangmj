/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T>(array: &mut [T])
where
    T: PartialOrd + Copy,
{
    // bubble_sort(array);
    insertion_sort(array);
}

fn bubble_sort<T>(array: &mut [T])
where
    T: PartialOrd + Copy,
{
    let len = array.len();
    for i in 0..len {
        let s = i + 1;
        let mut item = array[i];
        for j in s..len {
            if item > array[j] {
                array.swap(i, j);
                item = array[i];
            }
        }
    }
}

fn insertion_sort<T>(array: &mut [T])
where
    T: PartialOrd + Copy,
{
    let len = array.len();
    if len > 1 {
        for m in 1..len {
            for i in 0..m {
                if array[i] > array[m] {
                    let tmp = array[m];
                    for t in (i+1..=m).rev() {
                        array[t] = array[t-1];
                    }
                    array[i] = tmp;
                }
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
