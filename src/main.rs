use env_logger;
use log::debug;
use std::{collections::HashMap, env, process::exit};

// 初始化拼音映射
fn init_pinyin_map() -> HashMap<&'static str, [&'static str; 4]> {
    HashMap::from([
        ("a", ["ā", "á", "ǎ", "à"]),
        ("o", ["ō", "ó", "ǒ", "ò"]),
        ("e", ["ē", "é", "ě", "è"]),
        ("i", ["ī", "í", "ǐ", "ì"]),
        ("u", ["ū", "ú", "ǔ", "ù"]),
        ("v", ["ǖ", "ǘ", "ǚ", "ǜ"]),
    ])
}

// 处理拼音字符串并返回转换后的结果
fn process_pinyin_string(pinyin_str: &str, pinyin_map: &HashMap<&'static str, [&'static str; 4]>) -> String {
    let mut result = String::new();
    let mut index = 0;

    while index < pinyin_str.len() {
        let char = pinyin_str.chars().nth(index).unwrap();
        
        // 尝试获取拼音字符串中下一个字符（即索引 + 1 位置的字符）
        let tone_index = pinyin_str
            .chars()
            .nth(index + 1) 
            .and_then(|c| c.to_digit(10)) 
            .map(|d| d as usize - 1); 

        // 如果有声调，则进行处理
        if let Some(ti) = tone_index {
            debug!("[line {}] tone_index:{}", line!(), ti);
            if ti >= 4 {
                println!("错误：声调应在1-4之间");
                exit(-1);
            }
            if let Some(tone_chars) = pinyin_map.get(char.to_string().as_str()) {
                debug!("[line {}] tone_index:{:?}", line!(), tone_chars);
                result.push_str(tone_chars[ti]);
            } else {
                result.push(char); 
            }
            index += 2; 
        } else {
            result.push(char); 
            index += 1;
        }
    }

    result
}

fn main() {
    env_logger::init();
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: pinyin \"<pinyin_str>\"");
        exit(0);
    }

    let pinyin_str = args[1].replace("\"", "");
    let pinyin_map = init_pinyin_map();

    // 调用处理拼音字符串的函数
    let result = process_pinyin_string(&pinyin_str, &pinyin_map);

    println!("{}", result);
}
