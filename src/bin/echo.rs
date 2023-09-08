use anyhow::{bail, Context, Result};
use notebook::flyer::*;
use serde::{Deserialize, Serialize};
use std::io::{StdoutLock, Write};

/// Use this module to specify the CLI cmd structure.
mod cmd {
    use clap::Parser;
    #[derive(Parser)]
    #[command(author, version, about, long_about = None)]
    pub(crate) struct App {
        #[arg(long, short)]
        /// true or false
        boolean: bool,
    }

    pub(crate) fn new_app() -> App {
        App::parse()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Payload {
    Echo {
        echo: String,
    },
    EchoOk {
        echo: String,
    },
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk,
}

pub struct EchoNode {
    pub id: usize,
}
impl Node<Payload> for EchoNode {
    fn step(&mut self, input: Message<Payload>, output: &mut StdoutLock) -> Result<()> {
        match input.body.payload {
            Payload::Echo { echo } => {
                let reply = Message {
                    src: input.dst,
                    dst: input.src,
                    body: Body {
                        id: Some(self.id),
                        in_reply_to: input.body.id,
                        payload: Payload::EchoOk { echo: echo },
                    },
                };
                serde_json::to_writer(&mut *output, &reply)
                    .context("serialize response to echo")?;
                output.write_all(b"\n").context("trailing newline")?;
                self.id += 1;
            }
            Payload::EchoOk { .. } => {}
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
    let _ = cmd::new_app();
    let _ = run_flyer(EchoNode { id: 0 })?;
    Ok(())
}
