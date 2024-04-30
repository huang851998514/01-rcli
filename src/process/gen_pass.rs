use rand::seq::SliceRandom;
use zxcvbn::zxcvbn;

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

pub fn process_genpass(
    length: u8,
    no_uppercase: bool,
    no_lowercase: bool,
    no_number: bool,
    no_symbol: bool,
) -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();
    if !no_uppercase {
        chars.extend_from_slice(UPPER);
        password.push(*UPPER.choose(&mut rng).expect("不可能为空"));
    }
    if !no_lowercase {
        chars.extend_from_slice(LOWER);
        password.push(*LOWER.choose(&mut rng).expect("不可能为空"));
    }
    if !no_number {
        chars.extend_from_slice(NUMBER);
        password.push(*NUMBER.choose(&mut rng).expect("不可能为空"));
    }
    if !no_symbol {
        chars.extend_from_slice(SYMBOL);
        password.push(*SYMBOL.choose(&mut rng).expect("不可能为空"));
    }
    for _ in 0..(length - password.len() as u8) {
        let s = chars.choose(&mut rng).expect("不可能为空");
        //u8支持copy 所以不需要clone
        password.push(*s);
    }
    password.shuffle(&mut rng);
    let pass = String::from_utf8(password)?;
    println!("{}", pass);

    //密码强度检测
    let result = zxcvbn(&pass, &[])?;
    eprintln!("密码强度：{}", result.score());

    Ok(())
}
