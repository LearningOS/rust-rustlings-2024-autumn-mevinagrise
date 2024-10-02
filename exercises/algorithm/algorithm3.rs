/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: PartialOrd>(array: &mut [T]){
	if array.len() <= 1 {
        return;
    }

    let mut stack: Vec<(usize, usize)> = Vec::new();
    stack.push((0, array.len() - 1));

    while let Some((low, high)) = stack.pop() {
        if low < high {
            let pivot_index = partition(array, low, high);

            // 根据 pivot 分成两部分，压入栈中处理
            if pivot_index > 0 {
                stack.push((low, pivot_index - 1)); // 左边部分
            }
            stack.push((pivot_index + 1, high)); // 右边部分
        }
    }
}

fn partition<T: PartialOrd>(array: &mut [T], low: usize, high: usize) -> usize {
    let pivot = high; // 选择最后一个元素作为基准
    let mut i = low; // i 用来表示小于 pivot 的区域的右边界

    for j in low..high {
        if array[j] < array[pivot] {
            array.swap(i, j); // 将小于 pivot 的元素移到前面
            i += 1;
        }
    }
    array.swap(i, high); // 将基准点放在正确位置
    i
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