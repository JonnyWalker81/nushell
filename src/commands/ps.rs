use crate::commands::WholeStreamCommand;
use crate::errors::ShellError;
use crate::object::process::process_dict;
use crate::prelude::*;
use sysinfo::SystemExt;

pub struct PS;

impl WholeStreamCommand for PS {
    fn name(&self) -> &str {
        "ps"
    }

    fn signature(&self) -> Signature {
        Signature::build("ps")
    }

    fn usage(&self) -> &str {
        "View current processes."
    }

    fn run(
        &self,
        args: CommandArgs,
        registry: &CommandRegistry,
    ) -> Result<OutputStream, ShellError> {
        ps(args, registry)
    }
}

fn ps(args: CommandArgs, _registry: &CommandRegistry) -> Result<OutputStream, ShellError> {
    let system;

    #[cfg(target_os = "linux")]
    {
        system = sysinfo::System::new();
    }

    #[cfg(not(target_os = "linux"))]
    {
        use sysinfo::RefreshKind;
        let mut sy = sysinfo::System::new_with_specifics(RefreshKind::new().with_processes());
        sy.refresh_processes();

        system = sy;
    }
    let list = system.get_process_list();

    let list = list
        .into_iter()
        .map(|(_, process)| process_dict(process, Tag::unknown_origin(args.call_info.name_span)))
        .collect::<VecDeque<_>>();

    Ok(list.from_input_stream())
}
