use tgbot_app::GLOBAL_CONFIG;
// use ferrisgram::error::{GroupIteration, Result};
use ferrisgram::ext::filters::message;
use ferrisgram::ext::handlers::{CommandHandler, MessageHandler};
use ferrisgram::ext::{Dispatcher, Updater};
// use ferrisgram::types::LinkPreviewOptions;
use ferrisgram::Bot;

// use ferrisgram::input_file::NamedFile;
// use tokio::fs::File;
// use tokio::io::AsyncReadExt;
mod handler;
mod start;
use handler::handler;
use start::start;

mod shell;
use shell::{c, ls, ping, shell};
mod ai;
use ai::chatgpt;

pub mod download;
pub use download::{yt_audio, ytdlp};

#[tokio::main]
async fn main() {
    // 获取配置文件信息
    let config = GLOBAL_CONFIG.clone();

    let bot_token = &config.telegram.bot_token;
    // 此函数创建一个新的机器人实例并相应地处理错误
    let bot = match Bot::new(bot_token, None).await {
        Ok(bot) => bot,
        Err(error) => panic!("无法创建bot: {}", error),
    };

    // 调度程序是更新程序内部功能的一部分
    // 您可以使用它来添加处理程序。
    let dispatcher = &mut Dispatcher::new(&bot);

    // add_handler method maps the provided handler in group 0 automatically
    // add_handler 方法自动将提供的处理程序映射到组 0 中
    dispatcher.add_handler(CommandHandler::new("start", start));
    // shell
    dispatcher.add_handler(CommandHandler::new("ls", ls));
    dispatcher.add_handler(CommandHandler::new("ping", ping));
    dispatcher.add_handler(CommandHandler::new("c", c));
    dispatcher.add_handler(CommandHandler::new("shell", shell));

    // ai
    dispatcher.add_handler(CommandHandler::new("chatgpt", chatgpt));

    // download
    dispatcher.add_handler(CommandHandler::new("ytdlp", ytdlp));

    // add_handler_to_group is used to map the provided handler to a group manually.
    // note that handler groups are processed in ascending order.
    dispatcher.add_handler_to_group(
        MessageHandler::new(
            handler,
            // This will restrict our echo function to the messages which
            // contain either text or a caption.
            message::Text::filter().or(message::Caption::filter()),
        ),
        1,
    );

    let mut updater = Updater::new(&bot, dispatcher);

    // This method will start long polling through the getUpdates method
    let _ = updater.start_polling(true).await;
}
