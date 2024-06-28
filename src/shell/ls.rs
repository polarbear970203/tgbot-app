use ferrisgram::{error::GroupIteration, ext::Context, Bot};
use tgbot_app::util::verify_telegram;
use tokio::process::Command;

use ferrisgram::error::Result;

pub async fn ls(bot: Bot, ctx: Context) -> Result<GroupIteration> {

    // Same logic as chat applies on unwrapping effective message here.
    let msg = ctx.effective_message.unwrap();
    let chat_id = msg.chat.id;
    if !verify_telegram(&chat_id.to_string()) {
        return Ok(GroupIteration::EndGroups);
    }

    let output = Command::new("/usr/bin/ls")
        .args(["-l", "-a", "-h"])
        .output()
        .await
        .expect("ls command failed")
        .stdout;

    bot.send_message(chat_id, String::from_utf8_lossy(&output).to_string())
        .send()
        .await?;

    Ok(GroupIteration::EndGroups)
}