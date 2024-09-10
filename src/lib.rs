#![cfg_attr(windows, feature(abi_vectorcall))]
use ext_php_rs::binary::Binary;
use ext_php_rs::binary_slice::BinarySlice;
use ext_php_rs::prelude::*;
use ext_php_rs::types::Zval;

use crate::class::stream::VecStream;
use crate::class::uri::Uri;

mod class;
mod util;

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
}
