#[macro_export]
macro_rules! to {
    ($name:ident, $from:expr, $to:expr) => {
        swc_core::ecma::transforms::testing::test_inline!(
            Default::default(),
            |_| swc_core::ecma::visit::as_folder($crate::CatchClauseVisitor::new()),
            $name,
            $from,
            $to
        );
    };
}
