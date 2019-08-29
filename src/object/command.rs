use crate::object::{TaggedDictBuilder, Value};
use crate::commands::command::Command;
use crate::prelude::*;


pub(crate) fn command_dict(command: Arc<Command>, tag: impl Into<Tag>) -> Tagged<Value> {
    let tag = tag.into();

    let mut cmd_dict = TaggedDictBuilder::new(tag);

    cmd_dict.insert("name", Value::string(command.name()));
    cmd_dict.insert("usage", Value::string(command.usage()));


    cmd_dict.into_tagged_value()
}
