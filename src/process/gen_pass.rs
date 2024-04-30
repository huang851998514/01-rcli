use rand::seq::SliceRandom;

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

pub fn process_genpass(
    length: u8,
    uppercase: bool,
    lowercase: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();
    if uppercase {
        chars.extend_from_slice(UPPER);
    }
    if lowercase {
        chars.extend_from_slice(LOWER);
    }
    if number {
        chars.extend_from_slice(NUMBER);
    }
    if symbol {
        chars.extend_from_slice(SYMBOL);
    }
    for _ in 0..length {
        let s = chars.choose(&mut rng).expect("不可能为空");
        //u8支持copy 所以不需要clone
        password.push(*s);
    }
    password.shuffle(&mut rng);
    let pass = String::from_utf8(password)?;
    println!("{}", pass);

    Ok(())
}
