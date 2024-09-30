// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.


#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        // 直接使用 None 时，不需要调用 unwrap
        println!("my_option is None");
    }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6,
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // 不需要 resize 操作，这里可以直接创建一个空的 Vec
    let empty_vec: Vec<i32> = Vec::new();
    println!("This Vec is empty, see? {:?}", empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // 交换两个值的正确方法
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
