use async_trait::async_trait;
use maelstrom::{done, protocol::Message, Node, Result, Runtime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub(crate) struct Handler {}

#[async_trait]
impl Node for Handler {
    async fn process(&self, runtime: Runtime, req: Message) -> Result<()> {
        match req.get_type() {
            "generate" => {
                let resp = Response::GenerateOk {
                    id: Uuid::new_v4().to_string(),
                };
                runtime.reply(req, resp).await
            }
            _ => done(runtime, req),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
enum Response {
    GenerateOk { id: String },
}
