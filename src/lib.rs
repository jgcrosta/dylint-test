#![feature(rustc_private)]
#![warn(unused_extern_crates)]

extern crate rustc_hir;
extern crate rustc_span;

use clippy_utils::diagnostics::span_lint_and_help;
use rustc_hir::intravisit::Visitor;
use rustc_hir::intravisit::{walk_expr, FnKind};
use rustc_hir::{Body, FnDecl, HirId};
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::Span;

dylint_linting::declare_late_lint! {
    /// ### What it does
    ///
    /// ### Why is this bad?
    ///
    /// ### Known problems
    /// Remove if none.
    ///
    /// ### Example
    /// ```rust
    /// // example code where a warning is issued
    /// ```
    /// Use instead:
    /// ```rust
    /// // example code that does not raise a warning
    /// ```
    pub TEST_LINT,
    Warn,
    "I hate println, use tracing motherfucker"
}

impl<'tcx> LateLintPass<'tcx> for TestLint {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        _: FnKind<'tcx>,
        _: &'tcx FnDecl<'_>,
        body: &'tcx Body<'_>,
        _: Span,
        _: HirId,
    ) {
        impl<'tcx> Visitor<'tcx> for IsPrintLn {
            fn visit_expr(&mut self, expr: &'tcx Expr<'_>) {
                if let ExprKind::If(..) = &expr.kind {
                    self.is_print_ln = true;
                    self.span = Some(expr.span);
                }

                walk_expr(self, expr);
            }
        }

        #[derive(Debug)]
        struct IsPrintLn {
            span: Option<Span>,
            is_print_ln: bool,
        }

        let mut is_print_ln = IsPrintLn {
            span: None,
            is_print_ln: false,
        };

        walk_expr(&mut is_print_ln, body.value);

        if is_print_ln.is_print_ln {
            span_lint_and_help(
                cx,
                TEST_LINT,
                is_print_ln.span.unwrap(),
                "Please dont use ifs",
                None,
                "This is another message",
            );
        }
    }
}

#[test]
fn ui() {
    dylint_testing::ui_test(
        env!("CARGO_PKG_NAME"),
        &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("ui"),
    );
}
