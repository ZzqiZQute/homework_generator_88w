use std::error::Error;
use std::fs::OpenOptions;
use std::io::{stdin, Write};
use std::path::PathBuf;
use std::process;

enum Lang {
    C,
    CC,
    Rust,
}

macro_rules! log_error {
    ($err: expr) => {
         println!("Error: {}", $err);
    }
}

fn calc_digit_name(mut num: u64) -> String {
    // 个 十 百 千 万 十万 百万 千万 亿 十亿 百亿 千亿 万亿 十万亿 百万亿 千万亿 亿亿 ~
    let mut output = String::new();
    while num > 8 {
        output.insert_str(0, "亿");
        num -= 8;
    }
    while num > 4 {
        output.insert_str(0, "万");
        num -= 4;
    }
    output.insert_str(0, ["", "十", "百", "千"][(num - 1) as usize]);
    if output.len() == 0 {
        output.push_str("个");
    }
    output
}

fn calc_len(num: u64) -> u64 {
    let mut i = 1;
    let mut res = 1;
    loop {
        if num / i == 0 {
            break res - 1;
        }
        i = i * 10;
        res = res + 1;
    }
}

fn revert_num(num: u64) -> String {
    num.to_string().chars().rev().collect::<String>()
}

fn pad_left(string: String, num: usize) -> String {
    let mut m = String::from(string);
    m.insert_str(0, " ".repeat(num).as_str());
    m
}

fn format_very_long(very_long_code: Vec<String>) -> String {
    very_long_code.into_iter().map(|x| {
        let mut temp = String::from(x);
        temp.insert_str(0, " ".repeat(8).as_str());
        temp.push_str("\r\n");
        temp
    }).collect::<String>()
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Input language(c, cc, rust) and length of the target number(e.g. c 5): ");
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    let args = input.trim().split(" ").collect::<Vec<_>>();
    if args.len() != 2 {
        log_error!("Invalid arguments");
        process::exit(-1);
    }
    let lang = match args[0].to_uppercase().as_str() {
        "C" => Some(Lang::C),
        "CC" => Some(Lang::CC),
        "RUST" => Some(Lang::Rust),
        _ => None
    };
    if lang.is_none() {
        log_error!("Language is not support");
        process::exit(-1);
    }
    let target_number_len = args[1].parse::<u32>()?;
    let mut filename = String::new();
    println!("Input code filename: ");
    stdin().read_line(&mut filename).unwrap();
    let path = PathBuf::new().join(filename.trim());
    let mut output_file = OpenOptions::new().create(true).write(true).open(path)?;
    match lang.unwrap() {
        Lang::C => {
            let mut very_long_code: Vec<String> = vec![];
            for i in 1..(10 as u64).pow(target_number_len) {
                very_long_code.push(format!("case {}:", i));
                let len = calc_len(i);
                very_long_code.push(pad_left(format!("printf(\"是个{}位数\\r\\n\");", len), 4));
                let s = i.to_string();
                for j in 1..=len {
                    let from = s.len() - j as usize;
                    very_long_code.push(pad_left(format!("printf(\"{}位数是：{}\\r\\n\");", calc_digit_name(j), &s[from..from + 1]), 4));
                }
                very_long_code.push(pad_left(format!("printf(\"倒过来是：{}\\r\\n\");", revert_num(i)), 4));
                very_long_code.push(pad_left(String::from("break;"), 4));
            }
            output_file.write(format!(r#"
#include <stdio.h>
int main () {{
    printf("请输入一个不多于{}位的正整数：\r\n");
    int x;
    scanf("%d", &x);
    switch (x) {{
{}    }}
}}
"#, target_number_len, format_very_long(very_long_code)).trim().as_bytes())?;
        }
        Lang::CC => {
            let mut very_long_code: Vec<String> = vec![];
            for i in 1..(10 as u64).pow(target_number_len) {
                very_long_code.push(format!("case {}:", i));
                let len = calc_len(i);
                very_long_code.push(pad_left(format!("cout << \"是个{}位数\" << endl;", len), 4));
                let s = i.to_string();
                for j in 1..=len {
                    let from = s.len() - j as usize;
                    very_long_code.push(pad_left(format!("cout << \"{}位数是：{}\" << endl;", calc_digit_name(j), &s[from..from + 1]), 4));
                }
                very_long_code.push(pad_left(format!("cout << \"倒过来是：{}\" << endl;", revert_num(i)), 4));
                very_long_code.push(pad_left(String::from("break;"), 4));
            }
            output_file.write(format!(r#"
#include <iostream>
using namespace std;
int main () {{
    cout << "请输入一个不多于{}位的正整数：" << endl;
    int x;
    cin >> x;
    switch (x) {{
{}    }}
}}
"#, target_number_len, format_very_long(very_long_code)).trim().as_bytes())?;
        }
        Lang::Rust => {
            let mut very_long_code: Vec<String> = vec![];
            for i in 1..(10 as u64).pow(target_number_len) {
                very_long_code.push(format!("{} => {{", i));
                let len = calc_len(i);
                very_long_code.push(pad_left(format!("println!(\"是个{}位数\");", len), 4));
                let s = i.to_string();
                for j in 1..=len {
                    let from = s.len() - j as usize;
                    very_long_code.push(pad_left(format!("println!(\"{}位数是：{}\");", calc_digit_name(j), &s[from..from + 1]), 4));
                }
                very_long_code.push(pad_left(format!("println!(\"倒过来是：{}\");", revert_num(i)), 4));
                very_long_code.push(String::from("}"));
            }
            output_file.write(format!(r#"
use std::io;
use std::error::Error;
fn main () -> Result<(), Box<dyn Error>> {{
    println!("请输入一个不多于{}位的正整数：");
    let mut x = String::new();
    io::stdin().read_line(&mut x)?;
    let num = x.trim().parse::<u32>()?;
    match num {{
{}        _ => ()
    }}
    Ok(())
}}
"#, target_number_len, format_very_long(very_long_code)).trim().as_bytes())?;
        }
    }
    Ok(())
}
