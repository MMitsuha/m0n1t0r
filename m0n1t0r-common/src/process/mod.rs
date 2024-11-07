mod execute;
mod interactive;
mod list;

use crate::Result as AppResult;
use remoc::{
    rch::bin::{Receiver, Sender},
    rtc,
};

#[rtc::remote]
pub trait Agent: Sync {
    async fn execute(&self, command: String, args: Vec<String>) -> AppResult<execute::Output> {
        execute::execute(command, args).await
    }

    async fn interactive(&self, command: String) -> AppResult<(Sender, Receiver, Receiver)> {
        interactive::interactive(command).await
    }

    async fn list(&self) -> AppResult<Vec<list::Process>> {
        list::list().await
    }
}
