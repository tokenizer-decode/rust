// compile-flags:-g

// === GDB TESTS ===================================================================================
// gdb-command:run
// gdb-command:whatis TEST
// gdb-check:type = u64
// gdb-command:whatis no_mangle_info::namespace::OTHER_TEST
// gdb-check:type = u64

// === LLDB TESTS ==================================================================================
// lldb-command:run
// lldb-command:expr TEST
// lldb-check: (unsigned long) $0 = 3735928559
// lldb-command:expr no_mangle_test::namespace::OTHER_TEST
// lldb-check: (unsigned long) $0 = 42

// === CDB TESTS ==================================================================================
// cdb-command: g
// Note: LLDB and GDB allow referring to items that are in the same namespace of the symbol
// we currently have a breakpoint on in an unqualified way. CDB does not, and thus we need to
// refer to it in a fully qualified way.
// cdb-command: dx a!no_mangle_info::TEST
// cdb-check: a!no_mangle_info::TEST : 0xdeadbeef [Type: unsigned __int64]
// cdb-command: dx a!no_mangle_info::namespace::OTHER_TEST
// cdb-check: a!no_mangle_info::namespace::OTHER_TEST : 0x2a [Type: unsigned __int64]

#[no_mangle]
pub static TEST: u64 = 0xdeadbeef;

pub mod namespace {
    pub static OTHER_TEST: u64 = 42;
}

pub fn main() {
    println!("TEST: {}", TEST);
    println!("OTHER TEST: {}", namespace::OTHER_TEST); // #break
}
