use swc_core::common::DUMMY_SP;
use swc_core::ecma::ast::{
    CallExpr, Callee, CatchClause, Expr, ExprOrSpread, ExprStmt, Ident, ImportDecl,
    ImportNamedSpecifier, ImportSpecifier, Module, ModuleDecl, ModuleItem, Pat, Stmt, Str,
};
use swc_core::ecma::visit::{as_folder, VisitMutWith};
use swc_core::ecma::{
    ast::Program,
    visit::{FoldWith, VisitMut},
};
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};

mod tests;

// import { weirwoodErrorReport } from '@common/utils/basicAbility/monitor';

const IMPORT_PACKAGE_NAME: &str = "@common/utils/basicAbility/monitor";
const IMPORT_FN_NAME: &str = "weirwoodErrorReport";

pub struct CatchClauseVisitor {
    pub has_catched: bool,
}

impl CatchClauseVisitor {
    pub fn new() -> CatchClauseVisitor {
        CatchClauseVisitor { has_catched: false }
    }
}

impl VisitMut for CatchClauseVisitor {
    fn visit_mut_module(&mut self, module: &mut Module) {
        module.visit_mut_children_with(self);
        if !self.has_catched {
            return;
        }
        let mut uv_import_decl = None;
        for item in &mut module.body {
            if let ModuleItem::ModuleDecl(ModuleDecl::Import(import_decl)) = item {
                if import_decl.src.value == IMPORT_PACKAGE_NAME {
                    uv_import_decl = Some(import_decl);
                    break;
                }
            }
        }

        if let Some(import_decl) = uv_import_decl {
            let mut uv_imported = false;
            for specifier in &import_decl.specifiers {
                if let ImportSpecifier::Named(named_specifier) = specifier {
                    if named_specifier.local.sym == IMPORT_FN_NAME {
                        uv_imported = true;
                        break;
                    }
                }
            }
            // 引入了包但未导入方法
            if !uv_imported {
                import_decl
                    .specifiers
                    .push(ImportSpecifier::Named(ImportNamedSpecifier {
                        span: DUMMY_SP,
                        local: Ident::new(IMPORT_FN_NAME.into(), DUMMY_SP),
                        imported: None,
                        is_type_only: false,
                    }));
            }
            //未引入包名和方法
        } else {
            let new_import_decl = ModuleDecl::Import(ImportDecl {
                span: DUMMY_SP,
                specifiers: vec![ImportSpecifier::Named(ImportNamedSpecifier {
                    span: DUMMY_SP,
                    local: Ident::new(IMPORT_FN_NAME.into(), DUMMY_SP),
                    imported: None,
                    is_type_only: false,
                })],
                src: Box::from(Str {
                    span: DUMMY_SP,
                    value: IMPORT_PACKAGE_NAME.into(),
                    raw: None,
                }),
                type_only: false,
                with: None,
                phase: Default::default(),
            });
            //   判断是否导入了模块

            module
                .body
                .insert(0, ModuleItem::ModuleDecl(new_import_decl));
        }
    }

    fn visit_mut_catch_clause(&mut self, catch_clause: &mut CatchClause) {
        self.has_catched = true;
        // 先判断catch里面使用已经使用了uvFn
        let mut has_used_fn = false;
        for stmt in &catch_clause.body.stmts {
            if let Stmt::Expr(expr) = stmt {
                if let Expr::Call(call_expr) = &*expr.expr {
                    if let Callee::Expr(callee_expr) = &call_expr.callee {
                        if let Expr::Ident(ident) = &**callee_expr {
                            if ident.sym == IMPORT_FN_NAME {
                                has_used_fn = true;
                                break;
                            }
                        }
                    }
                }
            }
        }
        // 如果没有使用过uvFn函数，那么插入uvFn(err)
        if !has_used_fn {
            let uv_ident = Expr::Ident(Ident::new(IMPORT_FN_NAME.into(), DUMMY_SP));
            let uv_callee = Callee::Expr(Box::new(uv_ident));
            let error_ident = match &catch_clause.param {
                Some(pat) => match &pat {
                    Pat::Ident(ident) => ident.id.clone(),
                    _ => return,
                },
                None => return,
            };

            // 构建uvFn调用表达式
            let uv_call_expr = Expr::Call(CallExpr {
                callee: uv_callee,
                args: vec![ExprOrSpread {
                    expr: Box::new(Expr::Ident(error_ident)),
                    spread: None,
                }],
                type_args: None,
                span: DUMMY_SP,
            });

            // 插入uvFn调用语句到catch块的开头
            let error_report_stmt = Stmt::Expr(ExprStmt {
                span: DUMMY_SP,
                expr: Box::new(uv_call_expr),
            });

            catch_clause.body.stmts.insert(0, error_report_stmt);
        }

        // 继续遍历catch块中的子节点
        catch_clause.visit_mut_children_with(self);
    }
}

// impl VisitMut for CatchClauseVisitor {}

/// 插件入口
#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.fold_with(&mut as_folder(CatchClauseVisitor::new()))
}
