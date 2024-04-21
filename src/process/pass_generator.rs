use rand::seq::SliceRandom;
use zxcvbn::zxcvbn;

const LOWER: &[u8] = b"abcdefghjkmnopqrstuvwxyz";
const UPPER: &[u8] = b"ABCDEFGHIJKLMNPQRSTUVWXYZ";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*?~";

// 使用 genpass 生成密码, 生成密码的时候可以指定密码的长度, 是否包含大写字母, 小写字母, 数字, 特殊字符等
// 生成的密码可以直接输出到 stdout
pub fn process(
    no_upper: bool,
    no_lower: bool,
    no_number: bool,
    no_symbol: bool,
    length: u8,
) -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();
    let mut password = Vec::with_capacity(length as usize);
    let mut chars = Vec::new();
    if !no_upper {
        chars.extend_from_slice(UPPER);
        password.push(*UPPER.choose(&mut rng).unwrap());
    }
    if !no_lower {
        chars.extend_from_slice(LOWER);
        password.push(*LOWER.choose(&mut rng).unwrap());
    }
    if !no_number {
        chars.extend_from_slice(NUMBER);
        password.push(*NUMBER.choose(&mut rng).unwrap());
    }
    if !no_symbol {
        chars.extend_from_slice(SYMBOL);
        password.push(*SYMBOL.choose(&mut rng).unwrap());
    }
    for _ in 0..(length - password.len() as u8) {
        password.push(*chars.choose(&mut rng).unwrap());
    }
    password.shuffle(&mut rng);
    let password = String::from_utf8(password).expect("Unreachable: all bytes are valid utf8");
    println!("{}", password);
    //use zxcvbn estimate password strength and outprint to stderr
    let result = zxcvbn(&password, &[]).unwrap();
    eprintln!("Estimated strength: {}", result.score());
    Ok(())
}
