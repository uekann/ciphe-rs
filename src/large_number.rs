use std::{
    cmp::max,
    fmt::{Display, Formatter},
    ops::{Add, Div, Mul, Sub},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LargeNumber {
    bytes: [u8; 256],
}

impl LargeNumber {
    pub fn new() -> Self {
        LargeNumber { bytes: [0; 256] }
    }
}

fn add_string_num_to_string_num(s1: String, s2: String) -> String {
    let mut result = String::new();
    let mut carry = 0;
    let mut i1 = s1.chars().rev();
    let mut i2 = s2.chars().rev();
    for _ in 0..max(s1.len(), s2.len()) {
        let c1 = i1.next().unwrap_or('0').to_digit(10).unwrap();
        let c2 = i2.next().unwrap_or('0').to_digit(10).unwrap();
        let sum = c1 + c2 + carry;
        result.push_str(&(sum % 10).to_string());
        carry = sum / 10;
    }
    if carry > 0 {
        result.push_str(&carry.to_string());
    }
    result.chars().rev().collect()
}

fn multiply_256_to_string_num(s: String) -> String {
    let mut result = String::new();

    let v = [6, 5, 2];
    for (i, vi) in v.iter().enumerate() {
        let mut carry = 0;

        let mut partial_result = String::new();
        let mut itr = s.chars().rev();
        for _ in 0..s.len() {
            let c1 = itr.next().unwrap().to_digit(10).unwrap();
            let product = c1 * vi + carry;
            partial_result.push_str(&(product % 10).to_string());
            carry = product / 10;
        }
        if carry > 0 {
            partial_result.push_str(&carry.to_string());
        }
        partial_result = partial_result.chars().rev().collect();
        for _ in 0..i {
            partial_result.push('0');
        }
        result = add_string_num_to_string_num(result, partial_result);
    }
    result
}

fn divide_string_num_by_256(s: String) -> (String, u8) {
    let mut result = String::new();
    let mut carry = 0;
    for c in s.chars() {
        let dividend = carry * 10 + c.to_digit(10).unwrap();
        result.push_str(&(dividend / 256).to_string());
        carry = dividend % 256;
    }
    while result.starts_with("0") {
        result = result.chars().skip(1).collect();
    }

    if result.is_empty() {
        result.push('0');
    }

    (result, carry as u8)
}

impl Display for LargeNumber {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let mut i = 255;
        while i > 0 && self.bytes[i] == 0 {
            i -= 1;
        }
        if i == 0 && self.bytes[i] == 0 {
            return write!(f, "0");
        }
        let mut result = self.bytes[i].to_string();
        while i > 0 {
            i -= 1;
            result = add_string_num_to_string_num(
                multiply_256_to_string_num(result),
                self.bytes[i].to_string(),
            );
        }
        write!(f, "{}", result)
    }
}

impl From<u128> for LargeNumber {
    fn from(n: u128) -> Self {
        let mut large_number = LargeNumber::new();
        let mut n = n;
        for i in 0..16 {
            large_number.bytes[i] = (n % 256) as u8;
            n /= 256;
        }
        large_number
    }
}

impl From<String> for LargeNumber {
    fn from(s: String) -> Self {
        let mut large_number = LargeNumber::new();
        let mut s = s;
        let mut i = 0;
        while !s.is_empty() {
            let (result, carry) = divide_string_num_by_256(s);
            large_number.bytes[i] = carry;
            s = result;
            i += 1;
        }
        large_number
    }
}

impl Add for LargeNumber {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut result = LargeNumber::new();
        let mut carry = 0;
        for i in 0..256 {
            let sum = self.bytes[i] as u16 + other.bytes[i] as u16 + carry;
            result.bytes[i] = (sum % 256) as u8;
            carry = sum / 256;
        }

        if carry > 0 {
            panic!("LargeNumber overflow in addition");
        }

        result
    }
}

impl Sub for LargeNumber {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut result = LargeNumber::new();
        let mut borrow = 0;
        for i in 0..256 {
            let diff = self.bytes[i] as i16 - other.bytes[i] as i16 - borrow;
            if diff < 0 {
                result.bytes[i] = (diff + 256) as u8;
                borrow = 1;
            } else {
                result.bytes[i] = diff as u8;
                borrow = 0;
            }
        }

        if borrow > 0 {
            panic!("LargeNumber underflow in subtraction");
        }

        result
    }
}

#[cfg(test)]
mod test {
    use crate::large_number::divide_string_num_by_256;

    use super::add_string_num_to_string_num;
    use super::multiply_256_to_string_num;
    use super::LargeNumber;

    #[test]
    fn test_add_string_num_to_string_num() {
        assert_eq!(
            add_string_num_to_string_num("1".to_string(), "1".to_string()),
            "2"
        );
        assert_eq!(
            add_string_num_to_string_num("1".to_string(), "9".to_string()),
            "10"
        );
        assert_eq!(
            add_string_num_to_string_num("9".to_string(), "1".to_string()),
            "10"
        );
        assert_eq!(
            add_string_num_to_string_num("9".to_string(), "9".to_string()),
            "18"
        );
        assert_eq!(
            add_string_num_to_string_num("99".to_string(), "1".to_string()),
            "100"
        );
        assert_eq!(
            add_string_num_to_string_num("1".to_string(), "99".to_string()),
            "100"
        );
        assert_eq!(
            add_string_num_to_string_num("99".to_string(), "99".to_string()),
            "198"
        );
        assert_eq!(
            add_string_num_to_string_num("999".to_string(), "1".to_string()),
            "1000"
        );
        assert_eq!(
            add_string_num_to_string_num("1".to_string(), "999".to_string()),
            "1000"
        );
        assert_eq!(
            add_string_num_to_string_num("999".to_string(), "999".to_string()),
            "1998"
        );
    }

    #[test]
    fn test_multiply_256_to_string_num() {
        assert_eq!(multiply_256_to_string_num("1".to_string()), "256");
        assert_eq!(multiply_256_to_string_num("9".to_string()), "2304");
        assert_eq!(multiply_256_to_string_num("99".to_string()), "25344");
        assert_eq!(multiply_256_to_string_num("999".to_string()), "255744");
    }

    #[test]
    fn test_divide_string_num_by_256() {
        assert_eq!(
            divide_string_num_by_256("0".to_string()),
            ("0".to_string(), 0)
        );
        assert_eq!(
            divide_string_num_by_256("1".to_string()),
            ("0".to_string(), 1)
        );
        assert_eq!(
            divide_string_num_by_256("9".to_string()),
            ("0".to_string(), 9)
        );
        assert_eq!(
            divide_string_num_by_256("99".to_string()),
            ("0".to_string(), 99)
        );
        assert_eq!(
            divide_string_num_by_256("999".to_string()),
            ("3".to_string(), 231)
        );
        assert_eq!(
            divide_string_num_by_256("6593511689".to_string()),
            ("25755905".to_string(), 9)
        );
    }

    #[test]
    fn test_display() {
        let mut large_number = LargeNumber::new();
        large_number.bytes[0] = 1;
        assert_eq!(large_number.to_string(), "1");
        large_number.bytes[0] = 9;
        assert_eq!(large_number.to_string(), "9");
        large_number.bytes[1] = 1;
        assert_eq!(large_number.to_string(), "265");
        large_number.bytes[2] = 1;
        assert_eq!(large_number.to_string(), "65801");
        large_number.bytes[3] = 1;
        assert_eq!(large_number.to_string(), "16843017");
        large_number.bytes[4] = 1;
        assert_eq!(large_number.to_string(), "4311810313");
        large_number.bytes[3] = 137;
        assert_eq!(large_number.to_string(), "6593511689");

        let mut large_number = LargeNumber::new();
        large_number.bytes[255] = 1;
        assert_eq!(large_number.to_string(), "126238304966058622268417487065116999845484776053576109500509161826268184136202698801551568013761380717534054534851164138648904527931605160527688095259563605939964364716019515983399209962459578542172100149937763938581219604072733422507180056009672540900709554109516816573779593326332288314873251559077853068444977864803391962580800682760017849589281937637993445539366428356761821065267423102149447628375691862210717202025241630303118559188678304314076943801692528246980959705901641444238894928620825482303431806955690226308773426829503900930529395181208739591967195841536053143145775307050594328881077553168201547776");
        large_number.bytes[0] = 9;
        large_number.bytes[1] = 1;
        large_number.bytes[2] = 1;
        large_number.bytes[4] = 1;
        large_number.bytes[3] = 137;
        assert_eq!(large_number.to_string(), "126238304966058622268417487065116999845484776053576109500509161826268184136202698801551568013761380717534054534851164138648904527931605160527688095259563605939964364716019515983399209962459578542172100149937763938581219604072733422507180056009672540900709554109516816573779593326332288314873251559077853068444977864803391962580800682760017849589281937637993445539366428356761821065267423102149447628375691862210717202025241630303118559188678304314076943801692528246980959705901641444238894928620825482303431806955690226308773426829503900930529395181208739591967195841536053143145775307050594328881077553174795059465");
    }
}
