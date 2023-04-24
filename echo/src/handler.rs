use async_trait::async_trait;
use maelstrom::{done, protocol::Message, Node, Result, Runtime};

pub(crate) struct Handler {}

#[async_trait]
impl Node for Handler {
    async fn process(&self, runtime: Runtime, req: Message) -> Result<()> {
        if req.get_type() == "echo" {
            let echo = req.body.clone().with_type("echo_ok");
            return runtime.reply(req, echo).await;
        }
        done(runtime, req)
    }
}
