use crate::cshadow as c;
use crate::host::context::{ThreadContext, ThreadContextObjs};
use crate::host::syscall_types::SysCallArgs;
use crate::host::syscall_types::{SyscallError, SyscallResult};

use nix::errno::Errno;

pub mod eventfd;
pub mod fcntl;
pub mod ioctl;
mod random;
pub mod socket;
pub mod unistd;

pub struct SyscallHandler {
    // Will eventually contain syscall handler state once migrated from the c handler
}

impl SyscallHandler {
    pub fn new() -> SyscallHandler {
        SyscallHandler {}
    }

    pub fn syscall(&self, ctx: &mut ThreadContext, args: &SysCallArgs) -> SyscallResult {
        match args.number {
            libc::SYS_getrandom => self.getrandom(ctx, args),
            _ => Err(SyscallError::from(Errno::ENOSYS)),
        }
    }
}

mod export {
    use crate::utility::notnull::notnull_mut_debug;

    use super::*;

    #[no_mangle]
    pub extern "C" fn rustsyscallhandler_new() -> *mut SyscallHandler {
        Box::into_raw(Box::new(SyscallHandler::new()))
    }

    #[no_mangle]
    pub extern "C" fn rustsyscallhandler_free(handler_ptr: *mut SyscallHandler) {
        if handler_ptr.is_null() {
            return;
        }
        unsafe {
            Box::from_raw(handler_ptr);
        }
    }

    #[no_mangle]
    pub extern "C" fn rustsyscallhandler_syscall(
        sys: *mut SyscallHandler,
        csys: *mut c::SysCallHandler,
        args: *const c::SysCallArgs,
    ) -> c::SysCallReturn {
        assert!(!sys.is_null());
        let sys = unsafe { &mut *sys };
        let mut objs = unsafe { ThreadContextObjs::from_syscallhandler(notnull_mut_debug(csys)) };
        sys.syscall(&mut objs.borrow(), unsafe { args.as_ref().unwrap() })
            .into()
    }
}
