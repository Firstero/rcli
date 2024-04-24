use rand::seq::SliceRandom;

const LOWER: &[u8] = b"abcdefghjkmnopqrstuvwxyz";
const UPPER: &[u8] = b"ABCDEFGHIJKLMNPQRSTUVWXYZ";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*?~";

// 设置一个 Generator 类型, 用于生成密码
pub struct Generator {
    no_upper: bool,
    no_lower: bool,
    no_number: bool,
    no_symbol: bool,
    charset: Vec<u8>,
}

impl Generator {
    // 实现 Generator 的 builder 实现, 每个 no_upper, no_lower, no_number, no_symbol 方法都会返回一个 &mut Self, 用于链式调用,且接受一个 bool 参数, 并且返回一个 &mut Self， charst 根据 no_upper, no_lower, no_number, no_symbol 的值来生成, 最后调用 build 方法生成一个 Generator
    pub fn new() -> Self {
        Self {
            no_upper: false,
            no_lower: false,
            no_number: false,
            no_symbol: false,
            charset: Vec::new(),
        }
    }

    pub fn no_lower(mut self, no_lower: bool) -> Self {
        self.no_lower = no_lower;
        self
    }

    pub fn no_number(mut self, no_number: bool) -> Self {
        self.no_number = no_number;
        self
    }

    pub fn no_symbol(mut self, no_symbol: bool) -> Self {
        self.no_symbol = no_symbol;
        self
    }

    pub fn no_upper(mut self, no_upper: bool) -> Self {
        self.no_upper = no_upper;
        self
    }

    pub fn build(mut self) -> Self {
        if !self.no_upper {
            self.charset.extend_from_slice(UPPER);
        }
        if !self.no_lower {
            self.charset.extend_from_slice(LOWER);
        }
        if !self.no_number {
            self.charset.extend_from_slice(NUMBER);
        }
        if !self.no_symbol {
            self.charset.extend_from_slice(SYMBOL);
        }
        self
    }

    // 生成密码, 接受一个长度参数, 返回一个 Vec<u8>
    pub fn generate(&self, length: u8) -> Vec<u8> {
        let mut rng = rand::thread_rng();
        let mut password = Vec::with_capacity(length as usize);
        if !self.no_upper {
            password.push(*UPPER.choose(&mut rng).unwrap());
        }
        if !self.no_lower {
            password.push(*LOWER.choose(&mut rng).unwrap());
        }
        if !self.no_number {
            password.push(*NUMBER.choose(&mut rng).unwrap());
        }
        if !self.no_symbol {
            password.push(*SYMBOL.choose(&mut rng).unwrap());
        }
        if length as usize <= password.len() {
            password.shuffle(&mut rng);
            password.drain(0..length as usize).collect()
        } else {
            for _ in 0..(length - password.len() as u8) {
                password.push(*self.charset.choose(&mut rng).unwrap());
            }
            password.shuffle(&mut rng);
            password
        }
    }
}

// 使用 genpass 生成密码, 生成密码的时候可以指定密码的长度, 是否包含大写字母, 小写字母, 数字, 特殊字符等
// 生成的密码可以直接输出到 stdout
pub fn process(
    no_upper: bool,
    no_lower: bool,
    no_number: bool,
    no_symbol: bool,
    length: u8,
) -> Vec<u8> {
    let generator = Generator::new()
        .no_upper(no_upper)
        .no_lower(no_lower)
        .no_number(no_number)
        .no_symbol(no_symbol)
        .build();
    generator.generate(length)
}
