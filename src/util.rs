use ferrisgram::Bot;
use once_cell::sync::Lazy;
use reqwest::Client;
use reqwest::ClientBuilder;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use tokio::process::Command;

use crate::GLOBAL_CONFIG;

// telegram单条消息长度不能超过4096个字符
pub const MESSAGE_LEN: usize = 4000;

/// 验证ID是否存在于配置文件中
#[inline]
pub fn verify_telegram(id: i64) -> bool {
    GLOBAL_CONFIG.telegram.ids.contains(&id)
}

/// 验证telegram-id的宏，减少样板代码
#[macro_export]
macro_rules! verify_telegram_id {
    ($chat_id:expr) => {
        if !$crate::util::verify_telegram($chat_id) {
            tklog::async_fatal!("未知TelegramID进入handler：", $chat_id);
            return Ok(GroupIteration::EndGroups);
        }
    };
}

pub static REQWEST_CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
    let mut req_builder = ClientBuilder::new();

    if !GLOBAL_CONFIG.reqwest.user_agent.is_empty() {
        req_builder = req_builder.user_agent(&GLOBAL_CONFIG.reqwest.user_agent);
    }

    if !GLOBAL_CONFIG.reqwest.proxy.is_empty() {
        req_builder = req_builder.proxy(reqwest::Proxy::all(&GLOBAL_CONFIG.reqwest.proxy).unwrap())
    }

    match req_builder.build() {
        Ok(client) => client,
        Err(e) => {
            // 处理客户端创建错误的情况，例如记录错误日志并采取适当的措施
            eprintln!("Error creating client: {}", e);
            panic!("Failed to create reqwest client");
        }
    }
});

/// 出现失败后向用户发送失败信息
#[inline]
pub async fn send_err_msg<T: Display>(bot: Bot, chat_id: i64, msg: T) {
    let _ = bot
        .send_message(chat_id, format!("错误：{}", msg))
        .parse_mode(String::from("markdown"))
        .send()
        .await;
}

/// 分段消息的发送，telegram单条最多4,096个字符
pub async fn chunks_msg<T: AsRef<str>>(bot: &Bot, chat_id: i64, message: T) {
    for chunk in message.as_ref().chars().collect::<Vec<char>>().chunks(4000) {
        let chunk_str: String = chunk.iter().collect();
        let _ = bot.send_message(chat_id, chunk_str).send().await;
    }
}

// 执行单条shell命令单参数并返回输出
pub async fn execute_one_shell(shell_cmd: String, arg: impl ToString) -> Result<String, String> {
    let output = Command::new(&shell_cmd)
        .arg(arg.to_string())
        .output()
        .await
        .unwrap()
        .stdout;

    Ok(String::from_utf8_lossy(&output).to_string())
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody {
    pub model: String,
    pub messages: Vec<Messages>,
    pub temperature: Option<f32>,
}

#[derive(Serialize, Deserialize)]
pub struct Messages {
    pub role: String,
    pub content: String,
}

pub async fn ai_q_s<T: Into<String>>(content: T) -> anyhow::Result<String> {
    let tg_content = content.into();
    let client = Client::new();
    let api_key = &GLOBAL_CONFIG.openai.api_key;
    let msg = Messages {
        role: "user".to_string(),
        content: tg_content.to_string(),
    };
    let request_body = RequestBody {
        model: GLOBAL_CONFIG.openai.model.clone(),
        messages: vec![msg],
        temperature: Some(0.7),
    };
    let res = client
        .post(&GLOBAL_CONFIG.openai.base_url)
        .header("Authorization", "Bearer ".to_string() + api_key)
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await
        .unwrap();
    let response_body = res.json::<serde_json::Value>().await.unwrap();
    let rep = response_body["choices"][0]["message"]["content"]
        .as_str()
        .unwrap()
        .trim_start_matches('"')
        .trim_end_matches('"');

    // let _ = bot.send_message(chat_id, rep.to_string()).parse_mode("markdown".to_string()).send().await.unwrap();
    Ok(rep.to_string())
}
