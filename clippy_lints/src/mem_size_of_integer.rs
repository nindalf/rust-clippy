use crate::utils::{match_def_path, paths, snippet, span_lint_and_sugg};
use if_chain::if_chain;
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateLintPass, LateContext};
use rustc_session::{declare_lint_pass, declare_tool_lint};

declare_clippy_lint! {
    /// **What it does:** Checks for usages of std::mem::size_of() * 8 with integer types
    /// 
    /// **Why is this bad?** Each integer type has a constant BITS that has this info
    ///
    /// **Known problems:** None.
    ///
    /// **Example:**
    ///
    /// ```rust
    /// std::mem::size_of::<usize>() * 8
    /// ```
    /// Use instead:
    /// ```rust
    /// usize::BITS
    /// ```
    pub MEM_SIZE_OF_INTEGER,
    nursery,
    "replacing size_of() with a constant"
}


const MEM_SIZE_OF_INTEGER_MSRV: RustcVersion = RustcVersion::new(1, 52, 0);

pub struct MemSizeOfInteger {
    msrv: Option<RustcVersion>,
}

impl MemSizeOfInteger {
    #[must_use]
    pub fn new(msrv: Option<RustcVersion>) -> Self {
        Self { msrv }
    }
}

declare_lint_pass!(MemSizeOfInteger => [MEM_SIZE_OF_INTEGER]);


impl<'tcx> LateLintPass<'tcx> for MemSizeOfInteger {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'_>) {
        if_chain! {
            if let ExprKind::MethodCall(ref path_segment, ref args, ref span, _) = expr.kind;
            if let ExprKind::Path(ref path) = path_segment.ident;
            if let Some(def_id) = cx.qpath_res(path, func.hir_id).opt_def_id();
            if match_def_path(cx, def_id, &paths::STD_FS_CREATE_DIR);
            then {
                span_lint_and_sugg(
                    cx,
                    CREATE_DIR,
                    expr.span,
                    "calling `std::fs::create_dir` where there may be a better way",
                    "consider calling `std::fs::create_dir_all` instead",
                    format!("create_dir_all({})", snippet(cx, args[0].span, "..")),
                    Applicability::MaybeIncorrect,
                )
            }
        }
    }
}
