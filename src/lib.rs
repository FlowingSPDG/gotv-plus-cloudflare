use worker::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[durable_object]
pub struct Sync {
    tick: u32,
    rtdelay: u8,
    rcvage: u8,
    fragment: u32,
    signup_fragment: u32,
    tps: u8,
    protocol: u8, // should be 4
}

#[durable_object]
impl DurableObject for Sync {
    fn new(state: State, _env: Env) -> Self {
        Self {
            tick: 0,
            rtdelay: 0,
            rcvage: 0,
            fragment: 0,
            signup_fragment: 0,
            tps: 0,
            protocol: 0,
        }
    }

    async fn fetch(&mut self, _req: Request) -> Result<Response> {
        Response::ok(serde_json::to_string(self)?)
    }
}


#[event(fetch, respond_with_errors)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    let router = Router::new();

    router
        // Request from CS:GO Server

        // Start request
        .post_async("/match/:token/:fragment_number/start", |_req, ctx| async move {
            // Store "START" fragment on R2
            if let Some(token) = ctx.param("token") {
                let namespace = ctx.durable_object(token)?;
                let stub = namespace.id_from_name(token)?.get_stub()?;
                // TODO: Upload fragment
                // TODO: Register Match into Durable Objects
            }
            Response::error("Bad Request", 400)
        })

        // Request from CS:GO Client
        .get_async("/match/:token/sync", |_req, ctx| async move {
            // Return sync JSON on DurableObjects
            if let Some(token) = ctx.param("token") {
                let namespace = ctx.durable_object("matches")?;
                let stub = namespace.id_from_name(token)?.get_stub()?;
                let resp = stub.fetch_with_str(token).await?.json::<Sync>().await?;
                return Response::from_json(&resp)
            }

            Response::error("Bad Request", 400)
        })
        .run(req, env).await
}