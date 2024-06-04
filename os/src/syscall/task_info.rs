use core::convert::TryInto;

//use crate::batch::taskinfo;
use crate::task::TaskInfo;

pub fn sys_task_info(id: usize, _ts: *mut TaskInfo) -> isize {
    //taskinfo()
    id.try_into().unwrap()
}
