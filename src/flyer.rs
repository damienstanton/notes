//! "Solving distributed systems challenges in Rust" by Jon Gjengset
//! [video][1]
//!
//! [1]: https://youtu.be/gboGyccRVXI?si=2cvzvTBkKPhk_YZI
#![allow(unused)]
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::io::{StdoutLock, Write};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Body {
    #[serde(rename = "msg_id")]
    id: Option<usize>,
    in_reply_to: Option<usize>,
    #[serde(flatten)]
    payload: Payload,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
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

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Message {
    src: String,
    #[serde(rename = "dest")]
    dst: String,
    body: Body,
}

struct EchoNode {
    id: usize,
}
impl EchoNode {
    pub fn step(&mut self, input: Message, output: &mut StdoutLock) -> Result<()> {
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

pub fn run_flyer() -> Result<()> {
    use std::io::{stdin, stdout};
    let stdin = stdin().lock();
    let mut stdout = stdout().lock();
    let inputs = serde_json::Deserializer::from_reader(stdin).into_iter::<Message>();

    let mut state = EchoNode { id: 0 };

    for input in inputs {
        let input = input.context("maelstrom input failure")?;
        state
            .step(input, &mut stdout)
            .context("node step failure")?;
    }

    Ok(())
}
