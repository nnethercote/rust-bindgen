/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UsesTemplateParameter<T> {
    pub t: T,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UsesTemplateParameter_AlsoUsesTemplateParameter<T> {
    pub also: T,
}
impl <T> Default for UsesTemplateParameter_AlsoUsesTemplateParameter<T> {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
impl <T> Default for UsesTemplateParameter<T> {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
