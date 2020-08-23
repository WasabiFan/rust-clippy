use rustc_lint::{EarlyLintPass, EarlyContext};
use rustc_session::{declare_lint_pass, declare_tool_lint};
use rustc_ast::ast::*;

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

impl EarlyLintPass for UnusedFiles {}
