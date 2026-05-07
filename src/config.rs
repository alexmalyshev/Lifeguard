/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use pyrefly_python::module_name::ModuleName;

use crate::pyrefly::sys_info::SysInfo;
use crate::traits::SysInfoExt;

#[derive(Debug, Clone)]
pub struct AnalysisConfig {
    pub sys_info: SysInfo,
    pub main_module: Option<ModuleName>,
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            sys_info: SysInfo::lg_default(),
            main_module: None,
        }
    }
}

impl AnalysisConfig {
    pub fn new(sys_info: SysInfo, main_module: Option<ModuleName>) -> Self {
        Self {
            sys_info,
            main_module,
        }
    }
}
