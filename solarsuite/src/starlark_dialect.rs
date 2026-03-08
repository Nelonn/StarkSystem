//! Starlark dialect interpreter for Solar package definitions
//!
//! This module uses starlark-rust's AST parsing to interpret .bazon files.

use crate::package::PackageContext;
use starlark::syntax::{AstModule, Dialect};
use starlark_syntax::syntax::module::AstModuleFields;
use starlark_syntax::syntax::ast::{AstExpr, AstStmt, AstLiteral};

/// Extract string content from a Starlark expression
fn extract_string(expr: &AstExpr) -> Option<String> {
    match &expr.node {
        starlark_syntax::syntax::ast::ExprP::Literal(AstLiteral::String(s)) => Some(s.node.clone()),
        _ => None,
    }
}

/// Extract list of strings from a Starlark expression
fn extract_string_list(expr: &AstExpr) -> Vec<String> {
    match &expr.node {
        starlark_syntax::syntax::ast::ExprP::List(list) => {
            list.iter()
                .filter_map(|e| extract_string(e))
                .collect()
        }
        _ => Vec::new(),
    }
}

/// Parse a function call expression
fn parse_call(expr: &AstExpr) -> Option<(&str, Vec<&AstExpr>)> {
    match &expr.node {
        starlark_syntax::syntax::ast::ExprP::Call(func, args) => {
            if let starlark_syntax::syntax::ast::ExprP::Identifier(ident) = &func.node {
                let arg_exprs: Vec<&AstExpr> = args.args.iter()
                    .filter_map(|a| match &a.node {
                        starlark_syntax::syntax::ast::ArgumentP::Positional(expr) => Some(expr),
                        _ => None,
                    })
                    .collect();
                Some((ident.ident.as_str(), arg_exprs))
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Parse an assignment statement (name = value)
fn parse_assignment(stmt: &AstStmt) -> Option<(&str, &AstExpr)> {
    match &stmt.node {
        starlark_syntax::syntax::ast::StmtP::Assign(assign) => {
            match &assign.lhs.node {
                starlark_syntax::syntax::ast::AssignTargetP::Identifier(ident) => {
                    Some((ident.ident.as_str(), &assign.rhs))
                }
                _ => None,
            }
        }
        _ => None,
    }
}

/// Starlark parser for Solar package definitions
pub struct StarlarkParser;

impl StarlarkParser {
    /// Parse Starlark content using starlark-rust AST
    pub fn parse(content: &str, filepath: &str) -> Result<PackageContext, String> {
        // Parse the Starlark code using starlark-rust
        let ast = AstModule::parse(
            filepath,
            content.to_string(),
            &Dialect::Extended,
        ).map_err(|e| format!("Parse error: {}", e))?;

        let mut ctx = PackageContext::new();

        // Process each statement in the module
        Self::process_stmt(&mut ctx, ast.statement());

        Ok(ctx)
    }

    /// Process a statement
    fn process_stmt(ctx: &mut PackageContext, stmt: &AstStmt) {
        // Check for expression statements (function calls)
        if let starlark_syntax::syntax::ast::StmtP::Expression(expr) = &stmt.node {
            Self::process_call(ctx, expr);
            return;
        }

        // Check for assignments (prepare = "...", build = "...", etc.)
        if let Some((name, value)) = parse_assignment(stmt) {
            Self::process_assignment(ctx, name, value);
        }

        // Handle nested statements
        if let starlark_syntax::syntax::ast::StmtP::Statements(stmts) = &stmt.node {
            for s in stmts {
                Self::process_stmt(ctx, s);
            }
        }
    }

    /// Process a metadata assignment (description = "...", etc.)
    fn process_metadata_assignment(ctx: &mut PackageContext, name: &str, value: &AstExpr) {
        if let Some(text) = extract_string(value) {
            match name {
                "description" => ctx.set_description(text),
                "homepage" => ctx.set_homepage(text),
                "license" => ctx.set_license(text),
                _ => {}
            }
        }
    }

    /// Process a function call
    fn process_call(ctx: &mut PackageContext, expr: &AstExpr) {
        if let Some((func_name, args)) = parse_call(expr) {
            match func_name {
                "pkg" => {
                    if args.len() >= 2 {
                        if let (Some(name), Some(version)) = 
                            (extract_string(args[0]), extract_string(args[1])) {
                            ctx.pkg(name, version);
                        }
                    }
                }
                "description" => {
                    if let Some(text) = args.first().and_then(|a| extract_string(a)) {
                        ctx.set_description(text);
                    }
                }
                "homepage" => {
                    if let Some(url) = args.first().and_then(|a| extract_string(a)) {
                        ctx.set_homepage(url);
                    }
                }
                "license" => {
                    if let Some(lic) = args.first().and_then(|a| extract_string(a)) {
                        ctx.set_license(lic);
                    }
                }
                "arch" => {
                    if let Some(arg) = args.first() {
                        ctx.set_arch(extract_string_list(arg));
                    }
                }
                "depends" => {
                    if let Some(arg) = args.first() {
                        ctx.set_depends(extract_string_list(arg));
                    }
                }
                "optdepends" => {
                    if let Some(arg) = args.first() {
                        ctx.set_optdepends(extract_string_list(arg));
                    }
                }
                "conflicts" => {
                    if let Some(arg) = args.first() {
                        ctx.set_conflicts(extract_string_list(arg));
                    }
                }
                "provides" => {
                    if let Some(arg) = args.first() {
                        ctx.set_provides(extract_string_list(arg));
                    }
                }
                "replaces" => {
                    if let Some(arg) = args.first() {
                        ctx.set_replaces(extract_string_list(arg));
                    }
                }
                "backup" => {
                    if let Some(arg) = args.first() {
                        ctx.set_backup(extract_string_list(arg));
                    }
                }
                "source" => {
                    if let Some(arg) = args.first() {
                        ctx.set_source(extract_string_list(arg));
                    }
                }
                "sha256sums" => {
                    if let Some(arg) = args.first() {
                        ctx.set_sha256sums(extract_string_list(arg));
                    }
                }
                _ => {}
            }
        }
    }

    /// Process an assignment (for multiline strings)
    fn process_assignment(ctx: &mut PackageContext, name: &str, value: &AstExpr) {
        // First check for metadata assignments (description, homepage, license)
        Self::process_metadata_assignment(ctx, name, value);
        
        // Then check for script assignments (prepare, build, package)
        if let Some(script) = extract_string(value) {
            match name {
                "prepare" => ctx.set_prepare(script),
                "build" => ctx.set_build(script),
                "package" => ctx.set_package(script),
                _ => {}
            }
        }
    }
}
