use ferrisgram::error::Result;
use ferrisgram::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use ferrisgram::{error::GroupIteration, ext::Context, Bot};
use tgbot_app::util::verify_telegram;
use tokio::process::Command;


pub async fn dns(bot: Bot, ctx: Context) -> Result<GroupIteration> {
    // Same logic as chat applies on unwrapping effective message here.
    let msg = ctx.effective_message.unwrap();
    let chat_id = msg.chat.id;
    if !verify_telegram(chat_id) {
        return Ok(GroupIteration::EndGroups);
    }
    let cm = msg.text.unwrap();
    let d = if cm.starts_with('/') {
        cm[5..].trim()
    } else {
        cm[..].trim()
    };
    let button_start = InlineKeyboardButton::callback_button(
        "dnsrecon",
        format!("osint dns cb_dnsrecon {}", d).as_str(),
    );
    let button_google = InlineKeyboardButton::url_button("Google", "https://google.com");

    let button_baidu = InlineKeyboardButton::url_button("百度", "https://baidu.com");

    let dig_output = Command::new("dig")
        .arg(d)
        .output()
        .await
        .expect("dns命令执行失败")
        .stdout;

    bot.send_message(
        chat_id,
        format!("dig:{}", String::from_utf8_lossy(&dig_output)),
    )
    .send()
    .await?;

    let nslookup_output = Command::new("nslookup")
        .arg(d)
        .output()
        .await
        .expect("nslookup命令执行失败")
        .stdout;

    bot.send_message(
        chat_id,
        format!("nslookup:{}", String::from_utf8_lossy(&nslookup_output)),
    )
    .reply_markup(InlineKeyboardMarkup::new(vec![
        vec![button_start],
        vec![button_google, button_baidu],
    ]))
    .send()
    .await?;

    Ok(GroupIteration::EndGroups)
}

// 是否加一个将输出的内容报存到文件并发送呢？   内容过长
pub async fn cb_dnsenum(arg: &str, bot: Bot, chat_id: i64) -> Result<GroupIteration> {
    let dnsenum_output = Command::new("dnsenum")
        .args(["--reserver", arg]) // --reserver进行反向解析 加快扫描速度。保存文件的话可以不加此参数？
        .output()
        .await
        .expect("dnsenum命令执行失败")
        .stdout;

    bot.send_message(
        chat_id,
        format!("dnsenum:{}", String::from_utf8_lossy(&dnsenum_output)),
    )
    .send()
    .await?;

    Ok(GroupIteration::EndGroups)
}

pub async fn cb_dnsrecon(arg: &str, bot: Bot, chat_id: i64) -> Result<GroupIteration> {
    let dnsrecon_output = Command::new("dnsrecon")
        .args(["-d", arg])
        .output()
        .await
        .expect("dnsrecon命令执行失败")
        .stdout;

    let button_ai = InlineKeyboardButton::callback_button("AI总结", "AI总结 PROMPT_SHELL_OUTPUT");
    bot.send_message(
        chat_id,
        format!("dnsrecon:{}", String::from_utf8_lossy(&dnsrecon_output)),
    )
    .reply_markup(InlineKeyboardMarkup::new(vec![vec![button_ai]]))
    .send()
    .await?;

    Ok(GroupIteration::EndGroups)
}