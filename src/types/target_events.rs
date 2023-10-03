use crate::bindings::c_erc20_bindings::CErc20Events;
use crate::bindings::comptroller_bindings::ComptrollerEvents;

pub enum TargetEvents {
    ComptrollerEvent(ComptrollerEvents),
    CTokenEvent(CErc20Events),
}
