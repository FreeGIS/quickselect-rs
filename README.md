# quickselect
基于rust的选择算法。
## API
```
pub fn quick_select(
    arr: &mut Vec<f64>, 
    k: usize, 
    left: usize, 
    right: usize, 
    is_left_smallest: bool
)
```
- arr: 可变向量引用，该算法会对向量顺序修改。
- k：用于部分排序的中间索引。
- left: 排序起止索引，通常是0。
- right:排序终止索引，通常是arr.len()-1。
- is_left_smallest: true时，[left,k]是[left,right]中前k项最小的值；false时，[left,k]是[left,right]中前k项最大的值。
## 示例
```
  let mut arr_f64 = vec![
            65.0, 28.0, 59.0, 33.0, 21.0, 56.0, 22.0, 95.0, 50.0, 12.0, 90.0, 53.0, 28.0, 77.0,
            39.0,
        ];
        let length = arr_f64.len();
        quick_select(&mut arr_f64, 8, 0, length - 1, true);
        assert_eq!(
            arr_f64,
            vec![
                39.0, 28.0, 28.0, 33.0, 21.0, 12.0, 22.0, 50.0, 53.0, 56.0, 59.0, 65.0, 90.0, 77.0,
                95.0
            ]
        );
```
## 实际作用
常用于求数组中前k项最大、最小值。

