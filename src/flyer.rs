//! "Solving distributed systems challenges in Rust" by Jon Gjengset
//! [video][1]
//!
//! [1]: https://youtu.be/gboGyccRVXI?si=2cvzvTBkKPhk_YZI
use anyhow::{Context, Result};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::io::StdoutLock;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Body<Payload> {
    #[serde(rename = "msg_id")]
    pub id: Option<usize>,
    pub in_reply_to: Option<usize>,
    #[serde(flatten)]
    pub payload: Payload,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Init {
    pub node_id: String,
    pub node_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message<Payload> {
    pub src: String,
    #[serde(rename = "dest")]
    pub dst: String,
    pub body: Body<Payload>,
}

pub trait Node<Payload> {
    fn step(&mut self, input: Message<Payload>, output: &mut StdoutLock) -> Result<()>;
}

pub fn run_flyer<S, Payload>(mut state: S) -> Result<()>
where
    S: Node<Payload>,
    Payload: DeserializeOwned,
{
    use std::io::{stdin, stdout};
    let stdin = stdin().lock();
    let mut stdout = stdout().lock();
    let inputs = serde_json::Deserializer::from_reader(stdin).into_iter::<Message<Payload>>();

    for input in inputs {
        let input = input.context("maelstrom input failure")?;
        state
            .step(input, &mut stdout)
            .context("node step failure")?;
    }
    Ok(())
}
