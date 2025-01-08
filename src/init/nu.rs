use crate::model::{AliasConfig, AliasVisitor, VisitorAliasValue};

const NU_SCRIPT: &str = include_str!("./nu.nu");

#[inline]
pub fn print_alias<V: std::fmt::Display>(name: &str, value: V) {
    println!("alias {} = {}", name, value)
}

struct NuVisitor {}

impl AliasVisitor for NuVisitor {
    fn visit<'a>(&mut self, (name, value): (&'a str, VisitorAliasValue<'a>)) {
        print_alias(name, value);
    }
}

pub fn init(config: AliasConfig) {
    println!("{NU_SCRIPT}");

    config.visit_aliases("nu", &mut NuVisitor {});
}
