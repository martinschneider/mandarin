// My first attempt at Rust.
// Prints a given number in Chinese characters and pinyin.

fn main() {
    const DIGITS: [char;10] = ['零','一','二','三','四','五','六','七','八','九'];
    const DIGITS_PINYIN: &[&str] = &["líng ", "yī ", "èr ", "sān ", "sì ", "wǔ ", "liù ", "qī ", "bā ", "jiǔ "];
    const POWERS: [char;5] = [' ', '十','百','千','萬'];
    const POWERS_PINYIN: &[&str] = &["", "shí ", "bǎi ", "qiān ", "wàn "];

    let args = std::env::args().nth(1).unwrap_or("42".to_string());
    let number = args.parse::<usize>().ok().expect("Please enter a number between 0 and 99999");
    if number > 99999
    {
       println!("Please enter a number between 0 and 99999");
       return;
    }

    let mut x = number;
    let mut result: String = "".to_string();
    let mut pinyin: String = "".to_string();
    let mut i: usize = 0;
    let mut prev: bool = true;
    
    while x > 0
    {
       let power: String = POWERS[i].to_string();
       let power_pinyin: String = POWERS_PINYIN[i].to_string();
       let digit: usize = x % 10;
       if digit == 0
       {
          if !prev
          {
             result = [DIGITS[0].to_string(), result].join("");
             pinyin = [DIGITS_PINYIN[0].to_string(), pinyin].join("");
          }
          prev = true;
       }
       else
       {
          result = [DIGITS[x % 10].to_string(), power, result].join("");
          pinyin = [DIGITS_PINYIN[x % 10].to_string(), power_pinyin, pinyin].join("");
          prev = false;
       }
       x /= 10;
       i += 1;
    }
    println!("{} {}", result, pinyin);
}