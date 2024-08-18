#![allow(warnings)]
pub type REGISTER = ::std::os::raw::c_uchar;
#[repr(u8)]
pub enum Register {
	R0 = 0,
	R1 = 1,
	R2 = 2,
	R3 = 3,
	R4 = 4,
	R5 = 5,
	R6 = 6,
	R7 = 7,
}
pub type OPCODE = ::std::os::raw::c_uchar;
#[repr(u8)]
pub enum Opcode {
	NOP = 0,
	STOP = 1,
	RET = 2,
	IMOV = 3,
	LMOV = 4,
	FMOV = 5,
	DMOV = 6,
	IRCLR = 7,
	LRCLR = 8,
	FRCLR = 9,
	DRCLR = 10,
	ISR = 11,
	LSR = 12,
	IRSI = 13,
	ILSI = 14,
	LRSI = 15,
	LLSI = 16,
	IRSA = 17,
	ILSA = 18,
	LRSA = 19,
	LLSA = 20,
	ISMLD = 21,
	ISMST = 22,
	LSMLD = 23,
	LSMST = 24,
	FSMLD = 25,
	FSMST = 26,
	DSMLD = 27,
	DSMST = 28,
	POPI = 29,
	POPA = 30,
	IPUSH = 31,
	ISTLD = 32,
	ISTST = 33,
	LPUSH = 34,
	LSTLD = 35,
	LSTST = 36,
	FPUSH = 37,
	FSTLD = 38,
	FSTST = 39,
	DPUSH = 40,
	DSTLD = 41,
	DSTST = 42,
	ALLOC = 43,
	FREE = 44,
	HELD = 45,
	HEST = 46,
	IADD = 47,
	ISUB = 48,
	IMULT = 49,
	IDIV = 50,
	IMOD = 51,
	IINC = 52,
	IDEC = 53,
	INEG = 54,
	LADD = 55,
	LSUB = 56,
	LMULT = 57,
	LDIV = 58,
	LMOD = 59,
	LINC = 60,
	LDEC = 61,
	LNEG = 62,
	FADD = 63,
	FSUB = 64,
	FMULT = 65,
	FDIV = 66,
	FINC = 67,
	FDEC = 68,
	FNEG = 69,
	DADD = 70,
	DSUB = 71,
	DMULT = 72,
	DDIV = 73,
	DINC = 74,
	DDEC = 75,
	DNEG = 76,
	LTOI = 77,
	FTOI = 78,
	DTOI = 79,
	ITOL = 80,
	FTOL = 81,
	DTOL = 82,
	ITOF = 83,
	LTOF = 84,
	DTOF = 85,
	ITOD = 86,
	LTOD = 87,
	FTOD = 88,
	JMP = 89,
	IJEZ = 90,
	IJNZ = 91,
	IJEQ = 92,
	IJNQ = 93,
	IJML = 94,
	IJEL = 95,
	IJMG = 96,
	IJEG = 97,
	LJEZ = 98,
	LJNZ = 99,
	LJEQ = 100,
	LJNQ = 101,
	LJML = 102,
	LJEL = 103,
	LJMG = 104,
	LJEG = 105,
	FJEZ = 106,
	FJNZ = 107,
	FJEQ = 108,
	FJNQ = 109,
	FJML = 110,
	FJEL = 111,
	FJMG = 112,
	FJEG = 113,
	DJEZ = 114,
	DJNZ = 115,
	DJEQ = 116,
	DJNQ = 117,
	DJML = 118,
	DJEL = 119,
	DJMG = 120,
	DJEG = 121,
	CALL = 122,
	LIBCALL = 123,
}
/* automatically generated by rust-bindgen 0.69.4 */

pub const __WORDSIZE: u32 = 64;
pub const __has_safe_buffers: u32 = 1;
pub const __DARWIN_ONLY_64_BIT_INO_T: u32 = 1;
pub const __DARWIN_ONLY_UNIX_CONFORMANCE: u32 = 1;
pub const __DARWIN_ONLY_VERS_1050: u32 = 1;
pub const __DARWIN_UNIX03: u32 = 1;
pub const __DARWIN_64_BIT_INO_T: u32 = 1;
pub const __DARWIN_VERS_1050: u32 = 1;
pub const __DARWIN_NON_CANCELABLE: u32 = 0;
pub const __DARWIN_SUF_EXTSN: &[u8; 14] = b"$DARWIN_EXTSN\0";
pub const __DARWIN_C_ANSI: u32 = 4096;
pub const __DARWIN_C_FULL: u32 = 900000;
pub const __DARWIN_C_LEVEL: u32 = 900000;
pub const __STDC_WANT_LIB_EXT1__: u32 = 1;
pub const __DARWIN_NO_LONG_LONG: u32 = 0;
pub const _DARWIN_FEATURE_64_BIT_INODE: u32 = 1;
pub const _DARWIN_FEATURE_ONLY_64_BIT_INODE: u32 = 1;
pub const _DARWIN_FEATURE_ONLY_VERS_1050: u32 = 1;
pub const _DARWIN_FEATURE_ONLY_UNIX_CONFORMANCE: u32 = 1;
pub const _DARWIN_FEATURE_UNIX_CONFORMANCE: u32 = 3;
pub const __has_ptrcheck: u32 = 0;
pub const __PTHREAD_SIZE__: u32 = 8176;
pub const __PTHREAD_ATTR_SIZE__: u32 = 56;
pub const __PTHREAD_MUTEXATTR_SIZE__: u32 = 8;
pub const __PTHREAD_MUTEX_SIZE__: u32 = 56;
pub const __PTHREAD_CONDATTR_SIZE__: u32 = 8;
pub const __PTHREAD_COND_SIZE__: u32 = 40;
pub const __PTHREAD_ONCE_SIZE__: u32 = 8;
pub const __PTHREAD_RWLOCK_SIZE__: u32 = 192;
pub const __PTHREAD_RWLOCKATTR_SIZE__: u32 = 16;
pub const INT8_MAX: u32 = 127;
pub const INT16_MAX: u32 = 32767;
pub const INT32_MAX: u32 = 2147483647;
pub const INT64_MAX: u64 = 9223372036854775807;
pub const INT8_MIN: i32 = -128;
pub const INT16_MIN: i32 = -32768;
pub const INT32_MIN: i32 = -2147483648;
pub const INT64_MIN: i64 = -9223372036854775808;
pub const UINT8_MAX: u32 = 255;
pub const UINT16_MAX: u32 = 65535;
pub const UINT32_MAX: u32 = 4294967295;
pub const UINT64_MAX: i32 = -1;
pub const INT_LEAST8_MIN: i32 = -128;
pub const INT_LEAST16_MIN: i32 = -32768;
pub const INT_LEAST32_MIN: i32 = -2147483648;
pub const INT_LEAST64_MIN: i64 = -9223372036854775808;
pub const INT_LEAST8_MAX: u32 = 127;
pub const INT_LEAST16_MAX: u32 = 32767;
pub const INT_LEAST32_MAX: u32 = 2147483647;
pub const INT_LEAST64_MAX: u64 = 9223372036854775807;
pub const UINT_LEAST8_MAX: u32 = 255;
pub const UINT_LEAST16_MAX: u32 = 65535;
pub const UINT_LEAST32_MAX: u32 = 4294967295;
pub const UINT_LEAST64_MAX: i32 = -1;
pub const INT_FAST8_MIN: i32 = -128;
pub const INT_FAST16_MIN: i32 = -32768;
pub const INT_FAST32_MIN: i32 = -2147483648;
pub const INT_FAST64_MIN: i64 = -9223372036854775808;
pub const INT_FAST8_MAX: u32 = 127;
pub const INT_FAST16_MAX: u32 = 32767;
pub const INT_FAST32_MAX: u32 = 2147483647;
pub const INT_FAST64_MAX: u64 = 9223372036854775807;
pub const UINT_FAST8_MAX: u32 = 255;
pub const UINT_FAST16_MAX: u32 = 65535;
pub const UINT_FAST32_MAX: u32 = 4294967295;
pub const UINT_FAST64_MAX: i32 = -1;
pub const INTPTR_MAX: u64 = 9223372036854775807;
pub const INTPTR_MIN: i64 = -9223372036854775808;
pub const UINTPTR_MAX: i32 = -1;
pub const SIZE_MAX: i32 = -1;
pub const RSIZE_MAX: i32 = -1;
pub const WINT_MIN: i32 = -2147483648;
pub const WINT_MAX: u32 = 2147483647;
pub const SIG_ATOMIC_MIN: i32 = -2147483648;
pub const SIG_ATOMIC_MAX: u32 = 2147483647;
pub const NCVM_OK: u32 = 0;
pub const NCVM_U32_NOT_32_BIT: u32 = 1;
pub const NCVM_U64_NOT_64_BIT: u32 = 2;
pub const NCVM_IS_BIG_ENDIAN: u32 = 3;
pub const NCVM_STACK_ALLOC_ERROR: u32 = 4;
pub const NCVM_INCOMPATIBLE_VERSION: u32 = 5;
pub const NCVM_LIB_FUNCTION_NOT_FOUND: u32 = 6;
pub const NCVM_BYTECODE_READ_ERROR: u32 = 7;
pub const NCVM_MIN_VERSION: u32 = 0;
pub const NCVM_VERSION: u32 = 0;
pub type int_least8_t = i8;
pub type int_least16_t = i16;
pub type int_least32_t = i32;
pub type int_least64_t = i64;
pub type uint_least8_t = u8;
pub type uint_least16_t = u16;
pub type uint_least32_t = u32;
pub type uint_least64_t = u64;
pub type int_fast8_t = i8;
pub type int_fast16_t = i16;
pub type int_fast32_t = i32;
pub type int_fast64_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast16_t = u16;
pub type uint_fast32_t = u32;
pub type uint_fast64_t = u64;
pub type __int8_t = ::std::os::raw::c_schar;
pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __int16_t = ::std::os::raw::c_short;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __int32_t = ::std::os::raw::c_int;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __int64_t = ::std::os::raw::c_longlong;
pub type __uint64_t = ::std::os::raw::c_ulonglong;
pub type __darwin_intptr_t = ::std::os::raw::c_long;
pub type __darwin_natural_t = ::std::os::raw::c_uint;
pub type __darwin_ct_rune_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub union __mbstate_t {
    pub __mbstate8: [::std::os::raw::c_char; 128usize],
    pub _mbstateL: ::std::os::raw::c_longlong,
}
#[test]
fn bindgen_test_layout___mbstate_t() {
    const UNINIT: ::std::mem::MaybeUninit<__mbstate_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__mbstate_t>(),
        128usize,
        concat!("Size of: ", stringify!(__mbstate_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__mbstate_t>(),
        8usize,
        concat!("Alignment of ", stringify!(__mbstate_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__mbstate8) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__mbstate_t),
            "::",
            stringify!(__mbstate8)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._mbstateL) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__mbstate_t),
            "::",
            stringify!(_mbstateL)
        )
    );
}
pub type __darwin_mbstate_t = __mbstate_t;
pub type __darwin_ptrdiff_t = ::std::os::raw::c_long;
pub type __darwin_size_t = ::std::os::raw::c_ulong;
pub type __darwin_va_list = __builtin_va_list;
pub type __darwin_wchar_t = ::std::os::raw::c_int;
pub type __darwin_rune_t = __darwin_wchar_t;
pub type __darwin_wint_t = ::std::os::raw::c_int;
pub type __darwin_clock_t = ::std::os::raw::c_ulong;
pub type __darwin_socklen_t = __uint32_t;
pub type __darwin_ssize_t = ::std::os::raw::c_long;
pub type __darwin_time_t = ::std::os::raw::c_long;
pub type __darwin_blkcnt_t = __int64_t;
pub type __darwin_blksize_t = __int32_t;
pub type __darwin_dev_t = __int32_t;
pub type __darwin_fsblkcnt_t = ::std::os::raw::c_uint;
pub type __darwin_fsfilcnt_t = ::std::os::raw::c_uint;
pub type __darwin_gid_t = __uint32_t;
pub type __darwin_id_t = __uint32_t;
pub type __darwin_ino64_t = __uint64_t;
pub type __darwin_ino_t = __darwin_ino64_t;
pub type __darwin_mach_port_name_t = __darwin_natural_t;
pub type __darwin_mach_port_t = __darwin_mach_port_name_t;
pub type __darwin_mode_t = __uint16_t;
pub type __darwin_off_t = __int64_t;
pub type __darwin_pid_t = __int32_t;
pub type __darwin_sigset_t = __uint32_t;
pub type __darwin_suseconds_t = __int32_t;
pub type __darwin_uid_t = __uint32_t;
pub type __darwin_useconds_t = __uint32_t;
pub type __darwin_uuid_t = [::std::os::raw::c_uchar; 16usize];
pub type __darwin_uuid_string_t = [::std::os::raw::c_char; 37usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_pthread_handler_rec {
    pub __routine: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    pub __arg: *mut ::std::os::raw::c_void,
    pub __next: *mut __darwin_pthread_handler_rec,
}
#[test]
fn bindgen_test_layout___darwin_pthread_handler_rec() {
    const UNINIT: ::std::mem::MaybeUninit<__darwin_pthread_handler_rec> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__darwin_pthread_handler_rec>(),
        24usize,
        concat!("Size of: ", stringify!(__darwin_pthread_handler_rec))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_pthread_handler_rec>(),
        8usize,
        concat!("Alignment of ", stringify!(__darwin_pthread_handler_rec))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__routine) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_pthread_handler_rec),
            "::",
            stringify!(__routine)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__arg) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_pthread_handler_rec),
            "::",
            stringify!(__arg)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__next) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_pthread_handler_rec),
            "::",
            stringify!(__next)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_attr_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 56usize],
}
#[test]
fn bindgen_test_layout__opaque_pthread_attr_t() {
    const UNINIT: ::std::mem::MaybeUninit<_opaque_pthread_attr_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_opaque_pthread_attr_t>(),
        64usize,
        concat!("Size of: ", stringify!(_opaque_pthread_attr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_opaque_pthread_attr_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_opaque_pthread_attr_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sig) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_attr_t),
            "::",
            stringify!(__sig)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__opaque) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_attr_t),
            "::",
            stringify!(__opaque)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_cond_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 40usize],
}
#[test]
fn bindgen_test_layout__opaque_pthread_cond_t() {
    const UNINIT: ::std::mem::MaybeUninit<_opaque_pthread_cond_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_opaque_pthread_cond_t>(),
        48usize,
        concat!("Size of: ", stringify!(_opaque_pthread_cond_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_opaque_pthread_cond_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_opaque_pthread_cond_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sig) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_cond_t),
            "::",
            stringify!(__sig)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__opaque) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_cond_t),
            "::",
            stringify!(__opaque)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_condattr_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 8usize],
}
#[test]
fn bindgen_test_layout__opaque_pthread_condattr_t() {
    const UNINIT: ::std::mem::MaybeUninit<_opaque_pthread_condattr_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_opaque_pthread_condattr_t>(),
        16usize,
        concat!("Size of: ", stringify!(_opaque_pthread_condattr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_opaque_pthread_condattr_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_opaque_pthread_condattr_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sig) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_condattr_t),
            "::",
            stringify!(__sig)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__opaque) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_condattr_t),
            "::",
            stringify!(__opaque)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_mutex_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 56usize],
}
#[test]
fn bindgen_test_layout__opaque_pthread_mutex_t() {
    const UNINIT: ::std::mem::MaybeUninit<_opaque_pthread_mutex_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_opaque_pthread_mutex_t>(),
        64usize,
        concat!("Size of: ", stringify!(_opaque_pthread_mutex_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_opaque_pthread_mutex_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_opaque_pthread_mutex_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sig) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_mutex_t),
            "::",
            stringify!(__sig)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__opaque) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_mutex_t),
            "::",
            stringify!(__opaque)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_mutexattr_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 8usize],
}
#[test]
fn bindgen_test_layout__opaque_pthread_mutexattr_t() {
    const UNINIT: ::std::mem::MaybeUninit<_opaque_pthread_mutexattr_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_opaque_pthread_mutexattr_t>(),
        16usize,
        concat!("Size of: ", stringify!(_opaque_pthread_mutexattr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_opaque_pthread_mutexattr_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_opaque_pthread_mutexattr_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sig) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_mutexattr_t),
            "::",
            stringify!(__sig)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__opaque) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_mutexattr_t),
            "::",
            stringify!(__opaque)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_once_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 8usize],
}
#[test]
fn bindgen_test_layout__opaque_pthread_once_t() {
    const UNINIT: ::std::mem::MaybeUninit<_opaque_pthread_once_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_opaque_pthread_once_t>(),
        16usize,
        concat!("Size of: ", stringify!(_opaque_pthread_once_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_opaque_pthread_once_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_opaque_pthread_once_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sig) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_once_t),
            "::",
            stringify!(__sig)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__opaque) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_once_t),
            "::",
            stringify!(__opaque)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_rwlock_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 192usize],
}
#[test]
fn bindgen_test_layout__opaque_pthread_rwlock_t() {
    const UNINIT: ::std::mem::MaybeUninit<_opaque_pthread_rwlock_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_opaque_pthread_rwlock_t>(),
        200usize,
        concat!("Size of: ", stringify!(_opaque_pthread_rwlock_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_opaque_pthread_rwlock_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_opaque_pthread_rwlock_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sig) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_rwlock_t),
            "::",
            stringify!(__sig)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__opaque) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_rwlock_t),
            "::",
            stringify!(__opaque)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_rwlockattr_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 16usize],
}
#[test]
fn bindgen_test_layout__opaque_pthread_rwlockattr_t() {
    const UNINIT: ::std::mem::MaybeUninit<_opaque_pthread_rwlockattr_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_opaque_pthread_rwlockattr_t>(),
        24usize,
        concat!("Size of: ", stringify!(_opaque_pthread_rwlockattr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_opaque_pthread_rwlockattr_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_opaque_pthread_rwlockattr_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sig) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_rwlockattr_t),
            "::",
            stringify!(__sig)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__opaque) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_rwlockattr_t),
            "::",
            stringify!(__opaque)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_t {
    pub __sig: ::std::os::raw::c_long,
    pub __cleanup_stack: *mut __darwin_pthread_handler_rec,
    pub __opaque: [::std::os::raw::c_char; 8176usize],
}
#[test]
fn bindgen_test_layout__opaque_pthread_t() {
    const UNINIT: ::std::mem::MaybeUninit<_opaque_pthread_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_opaque_pthread_t>(),
        8192usize,
        concat!("Size of: ", stringify!(_opaque_pthread_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_opaque_pthread_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_opaque_pthread_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sig) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_t),
            "::",
            stringify!(__sig)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__cleanup_stack) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_t),
            "::",
            stringify!(__cleanup_stack)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__opaque) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_t),
            "::",
            stringify!(__opaque)
        )
    );
}
pub type __darwin_pthread_attr_t = _opaque_pthread_attr_t;
pub type __darwin_pthread_cond_t = _opaque_pthread_cond_t;
pub type __darwin_pthread_condattr_t = _opaque_pthread_condattr_t;
pub type __darwin_pthread_key_t = ::std::os::raw::c_ulong;
pub type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t;
pub type __darwin_pthread_mutexattr_t = _opaque_pthread_mutexattr_t;
pub type __darwin_pthread_once_t = _opaque_pthread_once_t;
pub type __darwin_pthread_rwlock_t = _opaque_pthread_rwlock_t;
pub type __darwin_pthread_rwlockattr_t = _opaque_pthread_rwlockattr_t;
pub type __darwin_pthread_t = *mut _opaque_pthread_t;
pub type u_int8_t = ::std::os::raw::c_uchar;
pub type u_int16_t = ::std::os::raw::c_ushort;
pub type u_int32_t = ::std::os::raw::c_uint;
pub type u_int64_t = ::std::os::raw::c_ulonglong;
pub type register_t = i64;
pub type user_addr_t = u_int64_t;
pub type user_size_t = u_int64_t;
pub type user_ssize_t = i64;
pub type user_long_t = i64;
pub type user_ulong_t = u_int64_t;
pub type user_time_t = i64;
pub type user_off_t = i64;
pub type syscall_arg_t = u_int64_t;
pub type intmax_t = ::std::os::raw::c_long;
pub type uintmax_t = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Instruction {
    pub opcode: OPCODE,
    pub r1: u8,
    pub r2: u8,
    pub r3: u8,
}
#[test]
fn bindgen_test_layout_Instruction() {
    const UNINIT: ::std::mem::MaybeUninit<Instruction> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Instruction>(),
        4usize,
        concat!("Size of: ", stringify!(Instruction))
    );
    assert_eq!(
        ::std::mem::align_of::<Instruction>(),
        1usize,
        concat!("Alignment of ", stringify!(Instruction))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Instruction),
            "::",
            stringify!(opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).r1) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(Instruction),
            "::",
            stringify!(r1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).r2) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(Instruction),
            "::",
            stringify!(r2)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).r3) as usize - ptr as usize },
        3usize,
        concat!(
            "Offset of field: ",
            stringify!(Instruction),
            "::",
            stringify!(r3)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ThreadSettings {
    pub u32_reg_size: u8,
    pub u64_reg_size: u8,
    pub f32_reg_size: u8,
    pub f64_reg_size: u8,
    pub stack_size: ::std::os::raw::c_ulong,
    pub call_stack_size: ::std::os::raw::c_ulong,
}
#[test]
fn bindgen_test_layout_ThreadSettings() {
    const UNINIT: ::std::mem::MaybeUninit<ThreadSettings> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ThreadSettings>(),
        24usize,
        concat!("Size of: ", stringify!(ThreadSettings))
    );
    assert_eq!(
        ::std::mem::align_of::<ThreadSettings>(),
        8usize,
        concat!("Alignment of ", stringify!(ThreadSettings))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).u32_reg_size) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ThreadSettings),
            "::",
            stringify!(u32_reg_size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).u64_reg_size) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(ThreadSettings),
            "::",
            stringify!(u64_reg_size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).f32_reg_size) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(ThreadSettings),
            "::",
            stringify!(f32_reg_size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).f64_reg_size) as usize - ptr as usize },
        3usize,
        concat!(
            "Offset of field: ",
            stringify!(ThreadSettings),
            "::",
            stringify!(f64_reg_size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).stack_size) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ThreadSettings),
            "::",
            stringify!(stack_size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).call_stack_size) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ThreadSettings),
            "::",
            stringify!(call_stack_size)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ncvm_thread {
    pub vm: *mut ::std::os::raw::c_void,
    pub current_instr_p: *const Instruction,
    pub stack_p: *mut ::std::os::raw::c_void,
    pub call_stack_p: *mut ::std::os::raw::c_void,
    pub u32_registers: *mut u32,
    pub u64_registers: *mut u64,
    pub f32_registers: *mut f32,
    pub f64_registers: *mut f64,
}
#[test]
fn bindgen_test_layout_ncvm_thread() {
    const UNINIT: ::std::mem::MaybeUninit<ncvm_thread> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ncvm_thread>(),
        64usize,
        concat!("Size of: ", stringify!(ncvm_thread))
    );
    assert_eq!(
        ::std::mem::align_of::<ncvm_thread>(),
        8usize,
        concat!("Alignment of ", stringify!(ncvm_thread))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vm) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ncvm_thread),
            "::",
            stringify!(vm)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).current_instr_p) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ncvm_thread),
            "::",
            stringify!(current_instr_p)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).stack_p) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ncvm_thread),
            "::",
            stringify!(stack_p)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).call_stack_p) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(ncvm_thread),
            "::",
            stringify!(call_stack_p)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).u32_registers) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(ncvm_thread),
            "::",
            stringify!(u32_registers)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).u64_registers) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(ncvm_thread),
            "::",
            stringify!(u64_registers)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).f32_registers) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(ncvm_thread),
            "::",
            stringify!(f32_registers)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).f64_registers) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(ncvm_thread),
            "::",
            stringify!(f64_registers)
        )
    );
}
pub type ncvm_lib_function = ::std::option::Option<unsafe extern "C" fn(thread: *mut ncvm_thread)>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ncvm {
    pub inst_p: *mut Instruction,
    pub inst_count: ::std::os::raw::c_ulong,
    pub static_mem_p: *mut u8,
    pub static_mem_size: ::std::os::raw::c_ulong,
    pub main_thread_settings: ThreadSettings,
    pub lib_functions: *mut ncvm_lib_function,
}
#[test]
fn bindgen_test_layout_ncvm() {
    const UNINIT: ::std::mem::MaybeUninit<ncvm> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ncvm>(),
        64usize,
        concat!("Size of: ", stringify!(ncvm))
    );
    assert_eq!(
        ::std::mem::align_of::<ncvm>(),
        8usize,
        concat!("Alignment of ", stringify!(ncvm))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).inst_p) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ncvm),
            "::",
            stringify!(inst_p)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).inst_count) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ncvm),
            "::",
            stringify!(inst_count)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).static_mem_p) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ncvm),
            "::",
            stringify!(static_mem_p)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).static_mem_size) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(ncvm),
            "::",
            stringify!(static_mem_size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).main_thread_settings) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(ncvm),
            "::",
            stringify!(main_thread_settings)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).lib_functions) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(ncvm),
            "::",
            stringify!(lib_functions)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ByteCodeBlocksInfo {
    pub static_mem_idx: ::std::os::raw::c_ulong,
    pub inst_idx: ::std::os::raw::c_ulong,
}
#[test]
fn bindgen_test_layout_ByteCodeBlocksInfo() {
    const UNINIT: ::std::mem::MaybeUninit<ByteCodeBlocksInfo> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ByteCodeBlocksInfo>(),
        16usize,
        concat!("Size of: ", stringify!(ByteCodeBlocksInfo))
    );
    assert_eq!(
        ::std::mem::align_of::<ByteCodeBlocksInfo>(),
        8usize,
        concat!("Alignment of ", stringify!(ByteCodeBlocksInfo))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).static_mem_idx) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ByteCodeBlocksInfo),
            "::",
            stringify!(static_mem_idx)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).inst_idx) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ByteCodeBlocksInfo),
            "::",
            stringify!(inst_idx)
        )
    );
}
extern "C" {
    #[doc = "@brief Initialize VM\n@param inst_p Instructions array\n@param static_mem_p Static memory\n@return VM"]
    pub fn ncvm_init(
        vm: *mut ncvm,
        inst_p: *mut Instruction,
        static_mem_p: *mut u8,
        get_lib_function: ::std::option::Option<
            unsafe extern "C" fn(
                name: *const ::std::os::raw::c_char,
                lib_data_p: *mut ::std::os::raw::c_void,
            ) -> ncvm_lib_function,
        >,
        lib_data_p: *mut ::std::os::raw::c_void,
    ) -> u8;
}
extern "C" {
    #[doc = "@brief Load VM from bytecode data\n@param data_p Bytecode data\n@param data_size Bytecode data size\n@return VM"]
    pub fn ncvm_loadBytecodeData(
        vm: *mut ncvm,
        data_p: *const u8,
        data_size: ::std::os::raw::c_ulong,
        get_lib_function: ::std::option::Option<
            unsafe extern "C" fn(
                name: *const ::std::os::raw::c_char,
                lib_data_p: *mut ::std::os::raw::c_void,
            ) -> ncvm_lib_function,
        >,
        lib_data_p: *mut ::std::os::raw::c_void,
    ) -> u8;
}
extern "C" {
    #[doc = "@brief Load VM from bytecode stream\n@param get_next_n_bytes Function to get next n bytes\n@param data_p Data pointer\n@param ret_code Return code\n@return VM"]
    pub fn ncvm_loadBytecodeStream(
        vm: *mut ncvm,
        get_next_n_bytes: ::std::option::Option<
            unsafe extern "C" fn(
                n: ::std::os::raw::c_ulong,
                data_p: *mut ::std::os::raw::c_void,
            ) -> *const u8,
        >,
        data_p: *mut ::std::os::raw::c_void,
        get_lib_function: ::std::option::Option<
            unsafe extern "C" fn(
                name: *const ::std::os::raw::c_char,
                lib_data_p: *mut ::std::os::raw::c_void,
            ) -> ncvm_lib_function,
        >,
        lib_data_p: *mut ::std::os::raw::c_void,
    ) -> u8;
}
extern "C" {
    #[doc = "@brief Free VM\n@param vm VM"]
    pub fn ncvm_free(vm: *mut ncvm);
}
extern "C" {
    #[doc = "@param vm VM"]
    pub fn ncvm_execute(vm: *mut ncvm) -> u8;
}
extern "C" {
    pub fn ncvm_create_thread(
        thread: *mut ncvm_thread,
        vm: *mut ncvm,
        start_instr_p: *const Instruction,
        ext_stack_p: *mut u8,
        ext_stack_s: ::std::os::raw::c_ulong,
        settings: ThreadSettings,
    ) -> u8;
}
extern "C" {
    pub fn ncvm_thread_free(thread: *mut ncvm_thread);
}
extern "C" {
    pub fn ncvm_execute_thread_step(thread: *mut ncvm_thread) -> u8;
}
extern "C" {
    pub fn ncvm_execute_thread(thread: *mut ncvm_thread) -> u8;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ncvm_default_lib_loader {
    pub lib_handlers: *mut *mut ::std::os::raw::c_void,
    pub lib_handlers_count: ::std::os::raw::c_ulong,
}
#[test]
fn bindgen_test_layout_ncvm_default_lib_loader() {
    const UNINIT: ::std::mem::MaybeUninit<ncvm_default_lib_loader> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ncvm_default_lib_loader>(),
        16usize,
        concat!("Size of: ", stringify!(ncvm_default_lib_loader))
    );
    assert_eq!(
        ::std::mem::align_of::<ncvm_default_lib_loader>(),
        8usize,
        concat!("Alignment of ", stringify!(ncvm_default_lib_loader))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).lib_handlers) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ncvm_default_lib_loader),
            "::",
            stringify!(lib_handlers)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).lib_handlers_count) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ncvm_default_lib_loader),
            "::",
            stringify!(lib_handlers_count)
        )
    );
}
extern "C" {
    pub fn ncvm_default_lib_loader_init(
        lib_names: *mut *const ::std::os::raw::c_char,
        lib_names_count: ::std::os::raw::c_ulong,
    ) -> ncvm_default_lib_loader;
}
extern "C" {
    pub fn ncvm_default_get_lib_function(
        name: *const ::std::os::raw::c_char,
        loader: *mut ::std::os::raw::c_void,
    ) -> ncvm_lib_function;
}
extern "C" {
    pub fn ncvm_default_lib_function_loader_free(loader: *mut ncvm_default_lib_loader);
}
pub type __builtin_va_list = *mut ::std::os::raw::c_char;


