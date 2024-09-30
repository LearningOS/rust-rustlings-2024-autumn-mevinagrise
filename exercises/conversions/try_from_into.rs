// try_from_into.rs
//
// TryFrom is a simple and safe type conversion that may fail in a controlled
// way under some circumstances. Basically, this is the same as From. The main
// difference is that this should return a Result type instead of the target
// type itself. You can read more about it at
// https://doc.rust-lang.org/std/convert/trait.TryFrom.html
//
// Execute `rustlings hint try_from_into` or use the `hint` watch subcommand for
// a hint.

use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// 错误类型定义
#[derive(Debug, PartialEq)]
enum IntoColorError {
    // 长度不正确的切片
    BadLen,
    // 整数转换错误
    IntConversion,
}

// 检查值是否在 0..=255 范围内的辅助函数
fn check_range(value: i16) -> Result<u8, IntoColorError> {
    if value >= 0 && value <= 255 {
        Ok(value as u8)
    } else {
        Err(IntoColorError::IntConversion)
    }
}

// 元组实现
impl TryFrom<(i16, i16, i16)> for Color {
    type Error = IntoColorError;

    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        Ok(Color {
            red: check_range(tuple.0)?,
            green: check_range(tuple.1)?,
            blue: check_range(tuple.2)?,
        })
    }
}

// 数组实现
impl TryFrom<[i16; 3]> for Color {
    type Error = IntoColorError;

    fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
        Ok(Color {
            red: check_range(arr[0])?,
            green: check_range(arr[1])?,
            blue: check_range(arr[2])?,
        })
    }
}

// 切片实现
impl TryFrom<&[i16]> for Color {
    type Error = IntoColorError;

    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
        if slice.len() != 3 {
            return Err(IntoColorError::BadLen);
        }
        Ok(Color {
            red: check_range(slice[0])?,
            green: check_range(slice[1])?,
            blue: check_range(slice[2])?,
        })
    }
}

fn main() {
    // 测试元组转换
    let c1 = Color::try_from((183, 65, 14));
    println!("{:?}", c1);

    // 测试数组转换
    let c2: Result<Color, _> = [183, 65, 14].try_into();
    println!("{:?}", c2);

    // 测试切片转换
    let v = vec![183, 65, 14];
    let c3 = Color::try_from(&v[..]);
    println!("{:?}", c3);
    
    let c4: Result<Color, _> = (&v[..]).try_into();
    println!("{:?}", c4);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuple_out_of_range_positive() {
        assert_eq!(
            Color::try_from((256, 1000, 10000)),
            Err(IntoColorError::IntConversion)
        );
    }

    #[test]
    fn test_tuple_out_of_range_negative() {
        assert_eq!(
            Color::try_from((-1, -10, -256)),
            Err(IntoColorError::IntConversion)
        );
    }

    #[test]
    fn test_tuple_correct() {
        let c: Result<Color, _> = (183, 65, 14).try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }

    #[test]
    fn test_array_out_of_range_positive() {
        let c: Result<Color, _> = [1000, 10000, 256].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }

    #[test]
    fn test_array_correct() {
        let c: Result<Color, _> = [183, 65, 14].try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }

    #[test]
    fn test_slice_correct() {
        let v = vec![183, 65, 14];
        let c: Result<Color, _> = Color::try_from(&v[..]);
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }

    #[test]
    fn test_slice_excess_length() {
        let v = vec![0, 0, 0, 0];
        assert_eq!(Color::try_from(&v[..]), Err(IntoColorError::BadLen));
    }
}
