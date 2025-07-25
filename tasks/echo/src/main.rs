use maelstrom::{Result, Runtime};

pub(crate) fn main() -> Result<()> {
    Runtime::init(try_main())
}

async fn try_main() -> Result<()> {
    let handler = std::sync::Arc::new(Handler::default());
    Runtime::new().with_handler(handler).run().await
}

#[derive(Clone, Default)]
struct Handler {}

#[async_trait::async_trait]
impl maelstrom::Node for Handler {
    async fn process(&self, runtime: Runtime, req: maelstrom::protocol::Message) -> Result<()> {
        if req.get_type() == "echo" {
            let echo = req.body.clone().with_type("echo_ok");
            return runtime.reply(req, echo).await;
        }

        maelstrom::done(runtime, req)
    }
}

˝
