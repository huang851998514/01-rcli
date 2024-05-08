use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine,
};

use crate::{utils::read_data, Base64Format};

pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<String> {
    let mut reader = read_data(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let encode = match format {
        Base64Format::Standard => STANDARD.encode(buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(buf),
    };
    Ok(encode)
}

pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<Vec<u8>> {
    let mut reader = read_data(input)?;
    let mut buf_string = String::new();
    reader.read_to_string(&mut buf_string)?;
    let buf = buf_string.trim();

    let decode = match format {
        Base64Format::Standard => STANDARD.decode(buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf),
    }?;

    Ok(decode)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_process_encode() {
        let input = "Cargo.toml";
        // fixtures/b64.txt
        let format = crate::Base64Format::UrlSafe;
        let result = super::process_encode(input, format);
        assert!(result.is_ok());
    }

    #[test]
    fn test_process_decode() {
        let input = "fixtures/b64.txt";
        let format = crate::Base64Format::UrlSafe;
        let result = super::process_decode(input, format);
        assert!(result.is_ok());
    }
}
