var searchIndex = new Map(JSON.parse('[\
["os",{"doc":"The main module and entrypoint","t":"CHCCCCCQQHCCCCCCCSSSESSSFNNNNHNNNNHJFJFNNNNNNOONNHHNNHNNHNNNNNNNFNNNNNHNNNNNHHHHECFNNNNONNNNNSSFSSSONNNNCNHCHCNNNSHHHHHFFFFONNNNNNCONHNNNNONNNHNHNOOHNHNOOHCCONNNNNNNNNFNNNNNNOOONNNNHPPPFGPNNNNNNNNNNNOONNNNNNSSHHHFCHHOOHOFNNNNNONONNNO","n":["board","clear_bss","config","console","lang_items","loader","logging","print","println","rust_main","sbi","stacktrace","sync","syscall","task","timer","trap","CLOCK_FREQ","APP_BASE_ADDRESS","APP_SIZE_LIMIT","CLOCK_FREQ","KERNEL_STACK_SIZE","MAX_APP_NUM","USER_STACK_SIZE","Stdout","borrow","borrow_mut","from","into","print","try_from","try_into","type_id","write_str","panic","KERNEL_STACK","KernelStack","USER_STACK","UserStack","borrow","borrow","borrow_mut","borrow_mut","clone","clone","data","data","from","from","get_base_i","get_num_app","get_sp","get_sp","init_app_cx","into","into","load_apps","push_context","try_from","try_from","try_into","try_into","type_id","type_id","SimpleLogger","borrow","borrow_mut","enabled","flush","from","init","into","log","try_from","try_into","type_id","console_putchar","set_timer","shutdown","print_stack_trace","UPSafeCell","up","UPSafeCell","borrow","borrow_mut","exclusive_access","from","inner","into","new","try_from","try_into","type_id","SYSCALL_EXIT","SYSCALL_GET_TIME","SYSCALL_STATS","SYSCALL_TASKINFO","SYSCALL_WRITE","SYSCALL_YIELD","__private_field","borrow","borrow_mut","deref","from","fs","into","print_syscall_stats","process","syscall","taskinfo","try_from","try_into","type_id","FD_STDOUT","sys_write","sys_exit","sys_get_time","sys_yield","sys_get_taskinfo","TASK_MANAGER","TaskContext","TaskManager","TaskManagerInner","__private_field","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","context","current_task","deref","exit_current_and_run_next","find_next_task","from","from","from","inner","into","into","into","mark_current_exited","mark_current_exited","mark_current_suspended","mark_current_suspended","num_app","ra","run_first_task","run_first_task","run_next_task","run_next_task","s","sp","suspend_current_and_run_next","switch","task","tasks","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","TaskContext","borrow","borrow_mut","clone","from","goto_restore","into","ra","s","sp","try_from","try_into","type_id","zero_init","__switch","Exited","Ready","Running","TaskControlBlock","TaskStatus","UnInit","borrow","borrow","borrow_mut","borrow_mut","clone","clone","eq","from","from","into","into","task_cx","task_status","try_from","try_from","try_into","try_into","type_id","type_id","MSEC_PER_SEC","TICKS_PER_SEC","get_time","get_time_ms","set_next_trigger","TrapContext","context","enable_timer_interrupt","init","sepc","sstatus","trap_handler","x","TrapContext","app_init_context","borrow","borrow_mut","from","into","sepc","set_sp","sstatus","try_from","try_into","type_id","x"],"q":[[0,"os"],[17,"os::board"],[18,"os::config"],[24,"os::console"],[34,"os::lang_items"],[35,"os::loader"],[64,"os::logging"],[76,"os::sbi"],[79,"os::stacktrace"],[80,"os::sync"],[82,"os::sync::up"],[93,"os::syscall"],[113,"os::syscall::fs"],[115,"os::syscall::process"],[118,"os::syscall::taskinfo"],[119,"os::task"],[167,"os::task::context"],[181,"os::task::switch"],[182,"os::task::task"],[207,"os::timer"],[212,"os::trap"],[220,"os::trap::context"],[233,"core::fmt"],[234,"core::result"],[235,"core::any"],[236,"core::fmt"],[237,"log"],[238,"log"],[239,"core::option"]],"d":["Constants used in rCore for qemu","clear BSS segment","Constants used in rCore","SBI console driver, for text output","The panic handler","Loading user applications into memory","","print string macro","println string macro","the rust entry-point of os","SBI call wrappers","","Synchronization and interior mutability primitives","Implementation of syscalls","Task management implementation","RISC-V timer-related functionality","Trap handling functionality","","","","","","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Get base address of app i.","Get the total number of applications.","","","get app info with entry and sp and save <code>TrapContext</code> in …","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Load nth user app at [APP_BASE_ADDRESS + n * …","","","","","","","","","","","","","Returns the argument unchanged.","","Calls <code>U::from(self)</code>.","","","","","use sbi call to putchar in console (qemu uart handler)","use sbi call to set timer","use sbi call to shutdown the kernel","","","Uniprocessor interior mutability primitives","Wrap a static data structure inside it so that we are able …","","","Exclusive access inner data in UPSafeCell. Panic if the …","Returns the argument unchanged.","inner data","Calls <code>U::from(self)</code>.","User is responsible to guarantee that inner struct is only …","","","","","","","","","","","","","","Returns the argument unchanged.","File and filesystem-related syscalls","Calls <code>U::from(self)</code>.","a helper function to print syscall stats","Process management syscalls","handle syscall exception with <code>syscall_id</code> and other …","","","","","","write buf of length <code>len</code>  to a file with <code>fd</code>","task exits and submit an exit code","get time in milliseconds","current task gives up resources for other tasks","","Global variable: TASK_MANAGER","Task Context","The task manager, where all the tasks are managed.","Inner of Task Manager","","","","","","","","Implementation of <code>TaskContext</code>","id of current <code>Running</code> task","","exit current task,  then run next task","Find next task to run and return task id.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","use inner value to get mutable access","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","exit current task","Change the status of current <code>Running</code> task into <code>Exited</code>.","suspend current task","Change the status of current <code>Running</code> task into <code>Ready</code>.","total number of tasks","return address ( e.g. __restore ) of __switch ASM function","run first task","Run the first task in task list.","rust next task","Switch current <code>Running</code> task to the task we have found, or …","callee saved registers:  s 0..11","kernel stack pointer of app","suspend current task, then run next task","Rust wrapper around <code>__switch</code>.","Types related to task management","task list","","","","","","","","","","Task Context","","","","Returns the argument unchanged.","set task context {__restore ASM funciton, kernel stack, …","Calls <code>U::from(self)</code>.","return address ( e.g. __restore ) of __switch ASM function","callee saved registers:  s 0..11","kernel stack pointer of app","","","","init task context","Switch to the context of <code>next_task_cx_ptr</code>, saving the …","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","read the <code>mtime</code> register","get current time in milliseconds","set the next timer interrupt","Trap Context","","timer interrupt enabled","initialize CSR <code>stvec</code> as the entry of <code>__alltraps</code>","CSR sepc","CSR sstatus      ","handle an interrupt, exception, or system call from user …","general regs[0..31]","Trap Context","init app context","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","CSR sepc","set stack pointer to x_2 reg (sp)","CSR sstatus      ","","","","general regs[0..31]"],"i":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,6,6,6,6,0,6,6,6,6,0,0,0,0,0,10,11,10,11,10,11,10,11,10,11,0,0,10,11,0,10,11,0,10,10,11,10,11,10,11,0,14,14,14,14,14,0,14,14,14,14,14,0,0,0,0,0,0,0,18,18,18,18,18,18,18,18,18,18,0,0,0,0,0,0,20,20,20,20,20,0,20,0,0,0,0,20,20,20,0,0,0,0,0,0,0,0,0,0,25,26,31,25,26,31,25,0,31,25,0,26,26,31,25,26,26,31,25,0,26,0,26,26,28,0,26,0,26,28,28,0,0,0,31,26,31,25,26,31,25,26,31,25,0,28,28,28,28,28,28,28,28,28,28,28,28,28,0,30,30,30,0,0,30,29,30,29,30,29,30,30,29,30,29,30,29,29,29,30,29,30,29,30,0,0,0,0,0,0,0,0,0,13,13,0,13,0,13,13,13,13,13,13,13,13,13,13,13,13],"f":"`{{}b}```````{{}d}```````````````{ce{}{}}0{cc{}}1{fb}{c{{h{e}}}{}{}}0{cj{}}{{ln}A`}{Abd}````6666{AdAd}{AfAf}``77{AhAh}{{}Ah}{AdAh}{AfAh}3<<>{{AdAj}Ah}::::99`=={{AlAn}B`}{Alb}>{{}b}{ce{}{}}{{AlBb}b}??>{Ahb}0{B`d}4```33{{{Bd{c}}}{{Bf{c}}}{}}{cc{}}`5{c{{Bd{c}}}{}}{c{{h{e}}}{}{}}0{cj{}}```````88{Bh{{Bd{{Bj{Ah}}}}}}4`9:`{{Ah{Bj{Ah}}}Bl}`332`{{AhBnAh}Bl}{C`d}{{}Bl}00`````======``{CbCd}?{Cd{{Cf{Ah}}}}:::`???{{}b}{Cdb}10``1{Cdd}21``2```;;;;;;:::`{ce{}{}}0{ChCh}?{AhCh}2```>>={{}Ch}```````3333{CjCj}{ClCl}{{ClCl}B`}{cc{}}077``{c{{h{e}}}{}{}}000{cj{}}0``{{}Ah}0=``==``{AjAj}``{{AhAh}Aj}<<5<`{{AjAh}b}`554`","c":[],"p":[[1,"unit"],[1,"never"],[5,"Arguments",233],[6,"Result",234],[5,"TypeId",235],[5,"Stdout",24],[1,"str"],[8,"Result",233],[5,"PanicInfo",236],[5,"KernelStack",35],[5,"UserStack",35],[1,"usize"],[5,"TrapContext",220],[5,"SimpleLogger",64],[5,"Metadata",237],[1,"bool"],[5,"Record",237],[5,"UPSafeCell",82],[5,"RefMut",238],[5,"SYSCALL_STATS",93],[1,"array"],[1,"isize"],[1,"u8"],[1,"i32"],[5,"TASK_MANAGER",119],[5,"TaskManager",119],[6,"Option",239],[5,"TaskContext",167],[5,"TaskControlBlock",182],[6,"TaskStatus",182],[5,"TaskManagerInner",119]],"b":[]}]\
]'));
if (typeof exports !== 'undefined') exports.searchIndex = searchIndex;
else if (window.initSearch) window.initSearch(searchIndex);
