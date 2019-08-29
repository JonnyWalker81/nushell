use crate::commands::command::EvaluatedWholeStreamCommandArgs;
use crate::object::command_dict;
use crate::commands::cp::CopyArgs;
use crate::commands::mkdir::MkdirArgs;
use crate::commands::mv::MoveArgs;
use crate::commands::rm::RemoveArgs;
use crate::context::SourceMap;
use crate::prelude::*;
use crate::shell::shell::Shell;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct HelpShell {
    pub(crate) path: String,
    pub(crate) value: Tagged<Value>,
}

impl HelpShell {
    pub fn new(value: Tagged<Value>) -> HelpShell {
        HelpShell {
            path: "help".to_string(),
            value,
        }
    }

    fn commands(&self, registry: &CommandRegistry, tag: impl Into<Tag>) -> VecDeque<Tagged<Value>> {
        let tag = tag.into();
        let mut cmds = VecDeque::new();
        
        for cmd in registry.names() {

            let value = command_dict(
                            registry.get_command(&cmd).unwrap(), 
                            tag
                        );

            cmds.push_back(value);
        }

        cmds
    }
}

impl Shell for HelpShell {
    fn name(&self, source_map: &SourceMap) -> String {
        let origin_name = self.value.origin_name(source_map);
        format!(
            "{}",
            match origin_name {
                Some(x) => format!("{{{}}}", x),
                None => format!("<{}>", self.value.item.type_name(),),
            }
        )
    }

    fn homedir(&self) -> Option<PathBuf> {
        dirs::home_dir()
    }

    fn path(&self) -> String {
        self.path.clone()
    }

    fn set_path(&mut self, path: String) {
        let _ = std::env::set_current_dir(&path);
        self.path = path.clone();
    }

    fn ls(&self, args: EvaluatedWholeStreamCommandArgs, registry: &CommandRegistry) -> Result<OutputStream, ShellError> {
        Ok(self
            .commands(registry, args.name_span())
            .to_output_stream())
    }

    fn cd(&self, _args: EvaluatedWholeStreamCommandArgs) -> Result<OutputStream, ShellError> {
        Ok(OutputStream::empty())
    }

    fn cp(&self, _args: CopyArgs, _name: Span, _path: &str) -> Result<OutputStream, ShellError> {
        Ok(OutputStream::empty())
    }

    fn mv(&self, _args: MoveArgs, _name: Span, _path: &str) -> Result<OutputStream, ShellError> {
        Ok(OutputStream::empty())
    }

    fn mkdir(&self, _args: MkdirArgs, _name: Span, _path: &str) -> Result<OutputStream, ShellError> {
        Ok(OutputStream::empty())
    }

    fn rm(&self, _args: RemoveArgs, _name: Span, _path: &str) -> Result<OutputStream, ShellError> {
        Ok(OutputStream::empty())
    }

    fn complete(
        &self,
        _line: &str,
        _pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> Result<(usize, Vec<rustyline::completion::Pair>), rustyline::error::ReadlineError> {
        let mut completions = vec![];
        completions.push(rustyline::completion::Pair {
            display: "open".to_string(),
            replacement: "abrir".to_string(),
        });
        Ok((0, completions))
    }

    fn hint(&self, _line: &str, _pos: usize, _ctx: &rustyline::Context<'_>) -> Option<String> {
        None
    }
}
