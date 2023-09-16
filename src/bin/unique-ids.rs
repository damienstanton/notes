use anyhow::{bail, Context, Result};
use notebook::oss::flyer::{run_flyer, Body, Message, Node};
use serde::{Deserialize, Serialize};
use std::io::{StdoutLock, Write};
use ulid::Ulid;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Payload {
    Generate,
    GenerateOk {
        #[serde(rename = "id")]
        guid: String,
    },
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk,
}

pub struct UniqueNode {
    pub id: usize,
}
impl Node<Payload> for UniqueNode {
    fn step(&mut self, input: Message<Payload>, output: &mut StdoutLock) -> Result<()> {
        match input.body.payload {
            Payload::Generate => {
                let guid = Ulid::new().to_string();
                let reply = Message {
                    src: input.dst,
                    dst: input.src,
                    body: Body {
                        id: Some(self.id),
                        in_reply_to: input.body.id,
                        payload: Payload::GenerateOk { guid },
                    },
                };
                serde_json::to_writer(&mut *output, &reply)
                    .context("serialize response to generate")?;
                output.write_all(b"\n").context("trailing newline")?;
                self.id += 1;
            }
            Payload::GenerateOk { .. } => {
                bail!("should never receive generateok")
            }
            Payload::Init { .. } => {
                let reply = Message {
                    src: input.dst,
                    dst: input.src,
                    body: Body {
                        id: Some(self.id),
                        in_reply_to: input.body.id,
                        payload: Payload::InitOk,
                    },
                };
                serde_json::to_writer(&mut *output, &reply)
                    .context("serialize response to init")?;
                output.write_all(b"\n").context("trailing newline")?;
                self.id += 1;
            }
            Payload::InitOk => bail!("should never receive initok"),
        }
        Ok(())
    }
}
fn main() -> Result<()> {
    // TODO: ~ 59:29
    run_flyer(UniqueNode { id: 0 })?;
    Ok(())
}
