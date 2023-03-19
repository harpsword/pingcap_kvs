use std::{
    io::{BufWriter, Read, Write},
    process::CommandArgs,
};

use crate::{error::CustomizedError, Result};

use super::command::Command;

/// Codec
// pub trait Codec<R: Read> {
//     /// encode
//     fn encode(&self, commands: Vec<Command>) -> Result<Vec<u8>>;

//     // maybe should return iterator on Command?
//     fn decode(&self, data: Vec<u8>) -> Result<Command>;

//     fn new() -> Self;

//     fn decode_from_reader(&self, reader: R) -> impl Iterator<Item = CommandParseResult>;
// }

pub struct SerdeJsonCodec {}

type CommandParseResult = Result<Command>;

pub fn decode_from_reader<R: Read>(reader: R) -> impl Iterator<Item = CommandParseResult> {
    // a is an Iterator of std::result::Result<Command, serde_json::error::Error>
    let a = serde_json::Deserializer::from_reader(reader).into_iter::<Command>();
    let b = a.map(|v| v.map_err(|err| CustomizedError::from(err)));
    b
}

impl SerdeJsonCodec {
    pub fn encode(&self, commands: Vec<Command>) -> Result<Vec<u8>> {
        let res = Vec::new();
        let mut writer = BufWriter::new(res);
        for tmp_command in commands {
            serde_json::to_writer(&mut writer, &tmp_command)?;
        }
        return Ok(writer.into_inner()?);
    }

    pub fn decode(&self, data: Vec<u8>) -> Result<Command> {
        let command: Command = serde_json::from_slice(&data)?;
        Ok(command)
    }

    pub fn new() -> Self {
        return SerdeJsonCodec {};
    }

    pub fn decode_from_reader<R: Read>(
        &self,
        reader: R,
    ) -> impl Iterator<Item = CommandParseResult> {
        let a = serde_json::Deserializer::from_reader(reader).into_iter::<Command>();
        let b = a.map(|v| v.map_err(|err| CustomizedError::from(err)));
        b
    }
}
