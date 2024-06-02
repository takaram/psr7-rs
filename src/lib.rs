#![cfg_attr(windows, feature(abi_vectorcall))]
use crate::class::uri::Uri;
use ext_php_rs::prelude::*;

mod class;
mod util;

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
}
