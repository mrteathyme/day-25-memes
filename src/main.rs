use std::ops::Add;

fn main() {
    let input = "1100-
11-1=12012=
111==2=-022=01
20---
1=01=20
2=2=2=-21-=
1021-02=10==-
1-22=12001-0=222011
1=1-1
2-2=
211=211=22-=0=10
111-111=212--01
10-2-121
10=-210000
12=
1==2-22--==01-20-0
1-1==120
201=120100=1
1==2111=1=
1--0-=-01
2
2=2
1=-=0==010-=--021
1=-2-=1-2=00122-10
210--21--
112
1=0-22-10-2=1==0
12-12
1==01-=-
1-1
1=1
11=20102-001=0=-20
11=11==10
22210=-=
12001=-0
1=-00122200=
20==-00=2=201=-=
1-21101021-====-
2-2=2=0=1122
10=2
1-02-10
21220--=-01--1-1
1==02-02
221-=0
110
2=122-
21-0=1=-
20
2---2-2-01-1=1110
2-11-211
12-110022-
10101-20-
211-2211-10-2002
2--22==1=1122=-0000
1=0
10-
20===21=00222==1
21-110-11=1
12--1=02-121
1101=00--2==0
11=0-1
10101-1=21-1121100
1=1010=-222-
22=2200---2=2-02-=2
20=1021=2--2---=
10=20112
1-0-10-
1002002=-==0
220-110
1=--=21
10--0
122==2-1-
1-2=100-0-=101
2=1
21=0=111=11=000=-2
12121=0
1-210-0=
2-1=111=20-1-2-
1012-2-02=
1121=2=1==
1=101-
1-21110
1=2011-0-
1==0
1-==2==12011-
11-2=121==
20-121002=021==111
22220-2-02=22=
221101
22-=20=02-0-1=-11
1-=01=010-1-1101-0
1-1-=0==0=010
1-212211-=02-11=-
1=01=
2000=10=-122=00
1=1=21-=-
1-=011=--2-2=12
2000000=2212-00
22120-11-1-0-=-2
112=1---21-0111
12112220=110
121=0011-=1-0
222=-=02121
2-000010221=
1=0221=1==-=2010
1102====
210=1-02-=1102
2=-=-201102012
2-=21-=0=1
11022-00-2=1210
1120-0-1012001
2-
10==0--0-020110
1-=02122-0=0=
1=10=2002=120=1-1=
1-=11
1-11201102-
2-==11102-101
10
2-=2=
2==0-
111=--12=2
1=201
11200-1==2=001=--2
10---10--=0-2202=1";

    let mut number = SNAFUNumber::try_from(0).unwrap();
    for i in input.lines() {
        println!("{i}");
        let new_number = SNAFUNumber::try_from(i.trim()).unwrap();
        println!("{} + {}", number.get_decimal(), new_number.get_decimal());
        number = number + new_number; 
    }
    println!("{:#?}", String::from(number.clone()));
    println!("{:#?}", number.get_decimal());
}

#[derive(Clone, Debug)]
struct SNAFUNumber(Vec<SNAFUDigits>);

impl From<SNAFUNumber> for String {
   fn from(mut value: SNAFUNumber) -> String {
       let mut string = String::new();
       if value.0.last().unwrap() == &SNAFUDigits::Zero {value.0.pop();}
       let digits: Vec<SNAFUDigits> = value.0.into_iter().rev().collect();
       for digit in digits {
            string.push_str(&String::from(digit))
       }
       string
   } 
}

impl From<SNAFUDigits> for String {
    fn from(value: SNAFUDigits) -> String {
        match value {
            SNAFUDigits::Two => "2".into(),
            SNAFUDigits::One => "1".into(),
            SNAFUDigits::Zero => "0".into(),
            SNAFUDigits::Minus => "-".into(),
            SNAFUDigits::DoubleMinus => "=".into()
        }
    }
}

impl TryFrom<&str> for SNAFUNumber {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut digits = vec![];
        for i in (0..value.len()).rev() {
            digits.push(SNAFUDigits::try_from(&value[i..=i])?)
        }
        Ok(SNAFUNumber(digits))
    }
}

impl TryFrom<i64> for SNAFUNumber {
    type Error = String;
    fn try_from(mut value: i64) -> Result<Self, Self::Error> {
        let mut digits = vec![];
        let mut largest = 0;
        while 5_i64.pow(largest) < value {
            largest+=1;
        }
        for i in (0..=largest).rev() {
            digits.push(value/5_i64.pow(i));
            value -= 5_i64.pow(i) * (value/5_i64.pow(i));
        }
        digits = digits.into_iter().rev().collect();
        let mut snafu_digits = vec![];
        for i in 0..digits.len() {
            if digits[i] <= 2 {snafu_digits.push(SNAFUDigits::try_from(digits[i])?)} else {
                if i+2 > digits.len() {
                    let rem = if digits[i] % 5 > 0 { 1 } else { 0 };
                    digits.push(digits[i]/5+rem);
                    digits[i] -= 5;
                    snafu_digits.push(SNAFUDigits::try_from(digits[i])?)
                } else {
                    let rem = if digits[i] % 5 > 0 { 1 } else { 0 };
                    digits[i+1] += digits[i]/5+rem;
                    digits[i] -= 5;
                    snafu_digits.push(SNAFUDigits::try_from(digits[i])?)
                }
            }
        }
        Ok(SNAFUNumber(snafu_digits))
    }
}

impl SNAFUNumber {
    fn get_decimal(&self) -> i64 {
        let mut sum = 0;
        for (i, num) in self.0.iter().enumerate() {
            sum += 5_i64.pow(i as u32) * num.get_decimal_rep();
        }
        sum
    }
}

impl Add for SNAFUNumber {
    type Output = SNAFUNumber;
    fn add(self, rhs: Self) -> Self {
        SNAFUNumber::try_from(self.get_decimal() + rhs.get_decimal()).unwrap()
    }
}

#[derive(Debug,Clone, PartialEq)]
enum SNAFUDigits {
    Two,
    One,
    Zero,
    Minus,
    DoubleMinus
}

impl TryFrom<i64> for SNAFUDigits {
    type Error = String;
    fn try_from(value: i64) -> Result<SNAFUDigits, Self::Error> {
        match value {
            2 => Ok(SNAFUDigits::Two),
            1 => Ok(SNAFUDigits::One),
            0 => Ok(SNAFUDigits::Zero),
            -1 => Ok(SNAFUDigits::Minus),
            -2 => Ok(SNAFUDigits::DoubleMinus),
            _ => Err(format!("Failed to convert {value}")),
        }
    }
}

impl TryFrom<&str> for SNAFUDigits {
    type Error = String;
    fn try_from(value: &str) -> Result<SNAFUDigits, Self::Error> {
        match value {
            "2" => Ok(SNAFUDigits::Two),
            "1" => Ok(SNAFUDigits::One),
            "0"=> Ok(SNAFUDigits::Zero),
            "-" => Ok(SNAFUDigits::Minus),
            "=" => Ok(SNAFUDigits::DoubleMinus),
            _ => Err(format!("Failed to convert {}", value)),
        }
    }
}

impl SNAFUDigits {
    fn get_decimal_rep(&self) -> i64 {
        match self {
            Self::Two => 2,
            Self::One => 1,
            Self::Zero => 0,
            Self::Minus => -1,
            Self::DoubleMinus => -2
        }
    }
}

