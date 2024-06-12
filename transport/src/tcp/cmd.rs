//! Socket Control path commands.
use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

use phoenix_api::Handle;

type IResult<T> = Result<T, phoenix_api::Error>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Command {
    Bind(SocketAddr, u32),
    Accept(Handle),
    Connect(SocketAddr),
    RegMr(usize),
    SetSockOption(Handle),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CompletionKind {
    Bind(Handle),
    Accept(Handle),
    Connect(Handle),
    RegMr(Handle),
    SetSockOption,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Completion(pub IResult<CompletionKind>);
