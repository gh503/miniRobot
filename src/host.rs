use crate::info::hostinfo::HostInfo;
use crate::common::ds::RunningRole;

#[derive(Debug)]
pub struct Host {
    info: HostInfo,
    role: RunningRole,
}

impl Host {
    pub fn new() -> Host {
        Host {
            info: HostInfo::new(),
            role: RunningRole::Inquirer,
        }
    }

    pub fn hostinfo(&self) -> &HostInfo {
        &self.info
    }

    pub fn role(&self) -> &RunningRole {
        &self.role
    }
}
