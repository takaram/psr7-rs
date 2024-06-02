use ext_php_rs::zend::ClassEntry;

pub(crate) fn invalid_argument_exception() -> &'static ClassEntry {
    ClassEntry::try_find("InvalidArgumentException").unwrap()
}
