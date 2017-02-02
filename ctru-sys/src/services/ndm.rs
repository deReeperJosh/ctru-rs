/* automatically generated by rust-bindgen */

#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum NDM_ExclusiveState {
    EXCLUSIVE_STATE_NONE = 0,
    EXCLUSIVE_STATE_INFRASTRUCTURE = 1,
    EXCLUSIVE_STATE_LOCAL_COMMUNICATIONS = 2,
    EXCLUSIVE_STATE_STREETPASS = 3,
    EXCLUSIVE_STATE_STREETPASS_DATA = 4,
}
extern "C" {
    pub fn ndmuInit() -> Result;
    pub fn ndmuExit();
    pub fn ndmuEnterExclusiveState(state: NDM_ExclusiveState) -> Result;
    pub fn ndmuLeaveExclusiveState() -> Result;
}
use ::types::*;