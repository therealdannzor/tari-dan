//   Copyright 2024 The Tari Project
//   SPDX-License-Identifier: BSD-3-Clause

use crate::process_manager::Instance;

pub struct IndexerProcess {
    instance: Instance,
}

impl IndexerProcess {
    pub fn new(instance: Instance) -> Self {
        Self { instance }
    }

    pub fn instance(&self) -> &Instance {
        &self.instance
    }

    pub fn instance_mut(&mut self) -> &mut Instance {
        &mut self.instance
    }
}
