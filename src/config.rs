/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use pyrefly_python::module_name::ModuleName;
use ruff_python_ast::CmpOp;
use ruff_python_ast::Expr;
use ruff_python_ast::Stmt;
use ruff_python_ast::StmtIf;

use crate::pyrefly::sys_info::SysInfo;
use crate::traits::SysInfoExt;

fn is_name_eq_main(name_side: &Expr, main_side: &Expr) -> bool {
    matches!(name_side, Expr::Name(n) if n.id.as_str() == "__name__")
        && matches!(main_side, Expr::StringLiteral(s) if s.value.to_str() == "__main__")
}

fn is_name_main_guard(expr: &Expr) -> bool {
    let Expr::Compare(cmp) = expr else {
        return false;
    };
    if cmp.ops.len() != 1 || cmp.ops[0] != CmpOp::Eq {
        return false;
    }
    let Some(right) = cmp.comparators.first() else {
        return false;
    };
    is_name_eq_main(cmp.left.as_ref(), right) || is_name_eq_main(right, cmp.left.as_ref())
}

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

    pub fn lg_pruned_if_branches<'a>(
        &'a self,
        x: &'a StmtIf,
        current_module: ModuleName,
    ) -> impl Iterator<Item = (Option<&'a Expr>, &'a [Stmt])> + 'a {
        let main_module = self.main_module;
        let mut done = false;
        self.sys_info
            .pruned_if_branches(x)
            .filter_map(move |(test, body)| {
                if done {
                    return None;
                }
                if let Some(t) = test {
                    if let Some(main) = main_module {
                        if is_name_main_guard(t) {
                            if main == current_module {
                                done = true;
                                return Some((None, body));
                            } else {
                                return None;
                            }
                        }
                    }
                }
                Some((test, body))
            })
    }
}
