use rustc_lint::{EarlyLintPass, EarlyContext};
use rustc_session::{declare_lint_pass, declare_tool_lint};
use rustc_ast::ast::*;
use rustc_data_structures::fx::FxHashSet;
use walkdir::WalkDir;
use crate::utils::span_lint;
use rustc_span::{Span, source_map::DUMMY_SP, FileName};
use std::error::Error;
use if_chain::if_chain;

declare_clippy_lint! {
    /// **What it does:**
    ///
    /// **Why is this bad?**
    ///
    /// **Known problems:** None.
    ///
    /// **Example:**
    ///
    /// ```rust
    /// // example code where clippy issues a warning
    /// ```
    /// Use instead:
    /// ```rust
    /// // example code which does not raise clippy warning
    /// ```
    pub UNUSED_FILES,
    cargo,
    "Rust files within the crate directory which are omitted from module tree."
}

declare_lint_pass!(UnusedFiles => [UNUSED_FILES]);

impl UnusedFiles {
    fn get_all_crate_files(context: &EarlyContext) -> FxHashSet<String> {
        let source_map = context.sess.source_map();

        let mut visited = FxHashSet::default();
        for file in source_map.files().iter() {
            let path = context.sess.working_dir.0.join(std::path::Path::new(&file.name.to_string()));
            visited.insert(path.to_str().unwrap().to_string());
        }

        visited
    }

    fn get_unused_files(context: &EarlyContext) -> Option<Vec<String>> {
        let crate_files = Self::get_all_crate_files(context);

        let path = context.sess.local_crate_source_file.as_ref()?;

        let mut dir = path.clone();
        dir.pop();

        let mut unused_files = Vec::new();
        for entry in WalkDir::new(dir) {
            match entry {
                Ok(entry) => {
                    // TODO: unwrap
                    let path = entry.path();
                    if let Some(ext) = path.extension() {
                        let path_str = path.to_str().unwrap();
                        if ext == "rs" && !crate_files.contains(path_str) {
                            unused_files.push(path_str.to_string());
                        }
                    }
                },
                Err(e) => {
                    span_lint(context, UNUSED_FILES,
                            DUMMY_SP, &format!("Error walking crate directory: {:?}", e));
                }
            }
        }

        Some(unused_files)
    }

    //     let crate_files = Self::get_all_crate_files(context);

    //     let path = context.sess.local_crate_source_file.unwrap();
    //     let mut dir = path.clone();
    //     dir.pop();

    //     let unused_files = Vec::new();
    //     for entry in WalkDir::new(dir) {
    //         // TODO: unwrap
    //         let path = entry?.path();
    //         if let Some(ext) = path.extension() {
    //             let path_str = path.to_str().unwrap();
    //             if ext == "rs" && !crate_files.contains(path_str) {
    //                 unused_files.push(path_str.to_string());
    //             }
    //         }
    //     }
    // }
}

impl EarlyLintPass for UnusedFiles {
    fn check_crate(&mut self, context: &EarlyContext, _: &Crate) {
        // let crate_files = Self::get_all_crate_files(context);

        if_chain! {
            if let Some(unused_files) = Self::get_unused_files(context);
            if unused_files.len() > 0;
            then {
                span_lint(context, UNUSED_FILES, DUMMY_SP, 
                    &format!("Found {} files within the Cargo source directory which aren't part of the module tree:\n{}\n", unused_files.len(), unused_files.join("\n")));
            }
        }
    }
}
