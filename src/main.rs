use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_genpass, process_text_generate,
    process_text_sign, process_text_verify, Base64SubCommand, Cli, HttpSubCommand, SubCommand,
    TextSignFormat, TextSubCommand,
};
use zxcvbn::zxcvbn;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let cli_parse = Cli::parse();
    match cli_parse.cmd {
        // 将csv文件转换为其他类型文件
        SubCommand::Csv(option) => {
            let output = match &option.output {
                Some(output) => output.clone(),
                None => format!("output.{}", option.format),
            };
            process_csv(&option.input, output, option.format)?;
        }
        // 生成随机密码
        SubCommand::GenPass(option) => {
            let pass = process_genpass(
                option.length,
                option.no_uppercase,
                option.no_lowercase,
                option.no_number,
                option.no_symbol,
            )?;
            print!("{}", pass);

            //密码强度检测
            let result = zxcvbn(&pass, &[])?;
            eprintln!("密码强度：{}", result.score());
        }
        // base64编码解码
        SubCommand::Base64(sub_command) => match sub_command {
            // base64编码
            Base64SubCommand::Encode(option) => {
                let encode = process_encode(&option.input, option.format)?;
                print!("{}", encode);
            }
            // base64解码
            Base64SubCommand::Decode(option) => {
                let decode = process_decode(&option.input, option.format)?;
                //TODO: decode出来的不一定是string，这里先这样处理
                let decode = String::from_utf8(decode)?;
                let decode = decode.trim().to_string();
                print!("{}", decode);
            }
        },
        // 文本签名/验证签名
        SubCommand::Text(sub_command) => match sub_command {
            // 文本签名
            TextSubCommand::Sign(options) => {
                let signed = process_text_sign(&options.input, &options.key, options.format)?;
                print!("{}", signed);
            }
            // 文本验证签名
            TextSubCommand::Verify(options) => {
                let verify = process_text_verify(
                    &options.input,
                    &options.key,
                    &options.signature,
                    options.format,
                )?;
                println!("{}", verify);
            }
            // 生成签名用的key
            TextSubCommand::Generate(options) => {
                let key = process_text_generate(options.format)?;
                let path = options.output;
                match options.format {
                    // 生成blake3签名用的key
                    TextSignFormat::Blake3 => {
                        let name = path.join("blake3.txt");
                        std::fs::write(name, &key[0])?;
                    }
                    // 生成ed25519签名用的key
                    TextSignFormat::Ed25519 => {
                        std::fs::write(path.join("ed25519.sk"), &key[0])?;
                        std::fs::write(path.join("ed25519.pk"), &key[1])?;
                    }
                }
            }
        },
        // http服务
        SubCommand::Http(sub_command) => match sub_command {
            // 启动http文件服务
            HttpSubCommand::Serve(options) => {
                println!("{:?}", options);
            }
        },
    }
    Ok(())
}
