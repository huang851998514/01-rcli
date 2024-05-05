use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine,
};

pub fn process_encode(input: &str) -> anyhow::Result<()> {
    let encode = STANDARD.encode(input);
    println!("{}", encode);
    Ok(())
}

pub fn process_decode(input: &str) -> anyhow::Result<()> {
    let decode = STANDARD.decode(input)?;
    let decode = String::from_utf8(decode)?;
    println!("{}", decode);
    Ok(())
}
