// tests6.rs
//
// In this example we take a shallow dive into the Rust standard library's
// unsafe functions. Fix all the question marks and todos to make the test
// pass.
//
// Execute `rustlings hint tests6` or use the `hint` watch subcommand for a
// hint.


struct Foo {
    a: u128,
    b: Option<String>,
}

/// # Safety
///
/// `ptr` 必须指向一个有效的 `Box<Foo>`，调用者负责保证该指针的有效性和唯一性。
unsafe fn raw_pointer_to_box(ptr: *mut Foo) -> Box<Foo> {
    // SAFETY: 我们根据契约假定 `ptr` 是通过 `Box::into_raw` 转换而来，因此可以安全地使用 `Box::from_raw`。
    let mut ret: Box<Foo> = unsafe { Box::from_raw(ptr) };
    // 修改 `b` 字段为 `Some("hello".to_owned())`
    ret.b = Some("hello".to_owned());
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_success() {
        let data = Box::new(Foo { a: 1, b: None });

        let ptr_1 = &data.a as *const u128 as usize;
        // SAFETY: We pass an owned box of `Foo`.
        let ret = unsafe { raw_pointer_to_box(Box::into_raw(data)) };

        let ptr_2 = &ret.a as *const u128 as usize;

        assert!(ptr_1 == ptr_2);
        assert!(ret.b == Some("hello".to_owned()));
    }
}
