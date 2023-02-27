extern crate clap;
use ansi_term::Colour;
use clap::{App, Arg, SubCommand};
use regex::Regex;
use serde_json::Value;
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;
use chrono::{TimeZone, Local, NaiveDateTime};
use percent_encoding;

fn main() {
    let matches = App::new("toolx")
        .version("0.0.1")
        .author("Levi Xia <xiawenyang@bonbonbwork.com>")
        .about("A collection of common tools for the command line tools")
        .subcommand(
            SubCommand::with_name("s2s")
                .about("Replace text content with specified characters")
                .arg(
                    Arg::with_name("from")
                        .short('F')
                        .long("from")
                        .value_name("FROM")
                        .help("characters to be replaced [default \\n]")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("replace")
                        .short('r')
                        .long("replace")
                        .value_name("REPLACE")
                        .help("characters to replace [default ,]")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("splice")
                        .short('s')
                        .long("splice")
                        .value_name("SPLICE")
                        .help("splice specific strings at both ends")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("text")
                        .short('t')
                        .long("text")
                        .value_name("TEXT")
                        .help("content to be handler")
                        .takes_value(true)
                        .required_unless("edit"),
                )
                .arg(
                    Arg::with_name("edit")
                        .short('e')
                        .long("edit")
                        .help("open editor to edit the text")
                        .takes_value(false)
                        .required_unless("text"),
                ),
        )
        .subcommand(
            SubCommand::with_name("j2f").about("json format").arg(
                Arg::with_name("text")
                    .short('t')
                    .long("text")
                    .value_name("TEXT")
                    .help("content to be handler")
                    .takes_value(true)
                    .required_unless("edit"),
            )
            .arg(
                Arg::with_name("edit")
                    .short('e')
                    .long("edit")
                    .help("open editor to edit the text")
                    .takes_value(false)
                    .required_unless("text"),
            ),
        )
        .subcommand(
            SubCommand::with_name("u2t")
                .about("transfer unix time to timestamp")
                .arg(
                    Arg::with_name("text")
                        .short('t')
                        .long("text")
                        .value_name("TEXT")
                        .help("content to be handler")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("t2u")
                .about("transfer timestamp to unix time")
                .arg(
                    Arg::with_name("text")
                        .short('t')
                        .long("text")
                        .value_name("TEXT")
                        .help("content to be handler")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("u2e")
                .about("url encode")
                .arg(
                    Arg::with_name("text")
                        .short('t')
                        .long("text")
                        .value_name("TEXT")
                        .help("content to be handler")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("e2u")
                .about("url decode")
                .arg(
                    Arg::with_name("text")
                        .short('t')
                        .long("text")
                        .value_name("TEXT")
                        .help("content to be handler")
                        .takes_value(true),
                ),
        )
        .get_matches();

    // s2s命令执行详情
    match matches.subcommand() {
        Some(("s2s", sub_matches)) => {
            let from = sub_matches.value_of("from").unwrap_or("\n");
            let replace = sub_matches.value_of("replace").unwrap_or(",");
            let text = sub_matches.value_of("text").unwrap_or("");
            let splice = sub_matches.value_of("splice").unwrap_or("");

            let edit = sub_matches.is_present("edit");

            let contents = if edit {
                edit_text(text)
            } else {
                String::from(text)
            };
            let re = Regex::new(&from).unwrap();
            let result = re.replace_all(&contents, replace).to_string();

            // 如果result最后内容是replace，则去除
            let result = if result.ends_with(&replace) {
                result[0..result.len() - replace.len()].to_string()
            } else {
                result
            };

            // 如果splice不为空，则将result通过replace分隔成数组，然后在每个元素前后加上splice
            let result = if splice != "" {
                result
                    .split(replace)
                    .map(|s| format!("{}{}{}", splice, s, splice))
                    .collect::<Vec<String>>()
                    .join(replace)
            } else {
                result
            };
            println!("{}", result);
        }
        
        Some(("j2f", sub_matches)) => {
            let text = sub_matches.value_of("text").unwrap_or("");
            let edit = sub_matches.is_present("edit");

            let text = if edit {
                edit_text(text)
            } else {
                String::from(text)
            };
            let json: Value = match serde_json::from_str(text.to_string().as_str()) {
                Ok(v) => v,
                Err(e) => {
                    println!("json parse error: {}", e);
                    return;
                }
            };
            let formatted_json = serde_json::to_string_pretty(&json).unwrap();
            // 将JSON字符串中的键添加颜色并打印输出
            // 正则表达式，用于匹配JSON键
            let re = Regex::new(r#""\w+":\s*"#).unwrap();
            let colored_json = re.replace_all(&formatted_json, |caps: &regex::Captures<'_>| {
                Colour::Green.paint(caps[0].to_string()).to_string()
            });
            println!("{}", colored_json);
        }

        Some(("u2t", sub_matches)) => {
            let text = sub_matches.value_of("text").unwrap_or("");
            let unix_secs = match text.parse::<i64>() {
                Ok(v) => v,
                Err(e) => {
                    println!("parse error: {}", e);
                    return;
                }
            };
            let dt = Local.timestamp(unix_secs, 0);
            println!("{}", dt.format("%Y-%m-%d %H:%M:%S"));
        }

        Some(("t2u", sub_matches)) => {
            let text = sub_matches.value_of("text").unwrap_or("");
            let dt = match NaiveDateTime::parse_from_str(text, "%Y-%m-%d %H:%M:%S") {
                Ok(v) => v,
                Err(e) => {
                    println!("parse error: {}", e);
                    return;
                }
            };
            let unix_secs = Local.from_local_datetime(&dt).unwrap().timestamp();
            println!("{}", unix_secs);
        }

        Some(("u2e", sub_matches)) => {
            let text = sub_matches.value_of("text").unwrap_or("");
            let encoded = percent_encoding::utf8_percent_encode(text, percent_encoding::NON_ALPHANUMERIC);
            println!("{}", encoded);
        }

        Some(("e2u", sub_matches)) => {
            let text = sub_matches.value_of("text").unwrap_or("");
            let decoded = percent_encoding::percent_decode_str(text).decode_utf8_lossy();
            println!("{}", decoded);
        }
        _ => {}
    }
}

// 开启vim编辑器，写入信息
fn edit_text(initial_text: &str) -> String {
    // 开启临时文件
    let mut file = NamedTempFile::new().unwrap();
    // 将传入内容先行写入文件
    writeln!(file, "{}", initial_text).unwrap();
    // 获取文件路径
    let file_path = file.path();
    // 执行vim程序
    Command::new("vim")
        .arg(file.path())
        .spawn()
        .expect("open vim failed")
        .wait()
        .expect("open vim failed");
    // 读取数据
    std::fs::read_to_string(file_path).expect("read data from template file failed")
}
