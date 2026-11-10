#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    fn rename(__old: *const libc::c_char, __new: *const libc::c_char) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn mkstemp(__template: *mut libc::c_char) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buf_t {
    pub data: *mut uint8_t,
    pub len: size_t,
    pub cap: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct read_t {
    pub p: *const uint8_t,
    pub len: size_t,
    pub current_location: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct user_t {
    pub name: *const uint8_t,
    pub name_len: uint32_t,
    pub salt: *const uint8_t,
    pub num_hashes: uint32_t,
    pub name_hashes: *const uint8_t,
    pub nonce: *const uint8_t,
    pub ciphertext: *const uint8_t,
    pub ciphertext_len: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct db_t {
    pub raw: *mut uint8_t,
    pub raw_len: size_t,
    pub users: *mut user_t,
    pub num_users: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ufile_t {
    pub name_hash: *const uint8_t,
    pub len: uint32_t,
    pub data: *const uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct userfiles_t {
    pub pt: *mut uint8_t,
    pub files: *mut ufile_t,
    pub nfiles: uint32_t,
}
pub type action_t = libc::c_uint;
pub const ACT_WRITE: action_t = 4;
pub const ACT_READ: action_t = 3;
pub const ACT_CREATE: action_t = 2;
pub const ACT_REGISTER: action_t = 1;
pub const ACT_NONE: action_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct args_t {
    pub user: *const libc::c_char,
    pub key: *const libc::c_char,
    pub file: *const libc::c_char,
    pub in_0: *const libc::c_char,
    pub out: *const libc::c_char,
    pub text: *const libc::c_char,
    pub act: action_t,
}
#[no_mangle]
pub unsafe extern "C" fn win() {
    printf(b"Arbitrary access achieved!\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn buf_reserve(mut b: *mut buf_t, mut extra: size_t) -> libc::c_int {
    let mut need: size_t = 0;
    let mut cap: size_t = 0;
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    if extra.wrapping_add((*b).len) > 18446744073709551615 as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    need = ((*b).len).wrapping_add(extra);
    if need <= (*b).cap {
        return 0 as libc::c_int;
    }
    cap = if (*b).cap != 0 { (*b).cap } else { 256 as libc::c_int as libc::c_ulong };
    while cap < need {
        cap = if cap
            > (18446744073709551615 as libc::c_ulong)
                .wrapping_div(2 as libc::c_int as libc::c_ulong)
        {
            need
        } else {
            cap.wrapping_mul(2 as libc::c_int as libc::c_ulong)
        };
    }
    p = realloc((*b).data as *mut libc::c_void, cap) as *mut uint8_t;
    if p.is_null() {
        return -(1 as libc::c_int);
    }
    (*b).data = p;
    (*b).cap = cap;
    return 0 as libc::c_int;
}
unsafe extern "C" fn buf_put(
    mut b: *mut buf_t,
    mut src: *const libc::c_void,
    mut n: size_t,
) -> libc::c_int {
    if n == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    if buf_reserve(b, n) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    memcpy(((*b).data).offset((*b).len as isize) as *mut libc::c_void, src, n);
    (*b).len = ((*b).len as libc::c_ulong).wrapping_add(n) as size_t as size_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn buf_u32(mut b: *mut buf_t, mut value: uint32_t) -> libc::c_int {
    let mut t: [uint8_t; 4] = [0; 4];
    t[0 as libc::c_int as usize] = value as uint8_t;
    t[1 as libc::c_int as usize] = (value >> 8 as libc::c_int) as uint8_t;
    t[2 as libc::c_int as usize] = (value >> 16 as libc::c_int) as uint8_t;
    t[3 as libc::c_int as usize] = (value >> 24 as libc::c_int) as uint8_t;
    return buf_put(
        b,
        t.as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[uint8_t; 4]>() as libc::c_ulong,
    );
}
unsafe extern "C" fn buf_free(mut b: *mut buf_t) {
    free((*b).data as *mut libc::c_void);
    memset(
        b as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<buf_t>() as libc::c_ulong,
    );
}
unsafe extern "C" fn read_left(mut r: *const read_t) -> size_t {
    return ((*r).len).wrapping_sub((*r).current_location);
}
unsafe extern "C" fn read_file(
    mut path: *const libc::c_char,
    mut out: *mut *mut uint8_t,
    mut out_len: *mut size_t,
) -> libc::c_int {
    let mut fp: *mut FILE = fopen(path, b"rb\0" as *const u8 as *const libc::c_char);
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    let mut n: libc::c_long = 0;
    let mut rc: libc::c_int = -(1 as libc::c_int);
    if fp.is_null() {
        return -(1 as libc::c_int);
    }
    fseek(fp, 0 as libc::c_int as libc::c_long, 2 as libc::c_int);
    n = ftell(fp);
    fseek(fp, 0 as libc::c_int as libc::c_long, 0 as libc::c_int);
    if n >= 0 as libc::c_int as libc::c_long
        && {
            p = malloc(n as libc::c_ulong) as *mut uint8_t;
            !p.is_null()
        }
        && fread(
            p as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            n as libc::c_ulong,
            fp,
        ) == n as size_t
    {
        *out = p;
        *out_len = n as size_t;
        rc = 0 as libc::c_int;
    } else {
        free(p as *mut libc::c_void);
    }
    fclose(fp);
    return rc;
}
unsafe extern "C" fn write_file(
    mut path: *const libc::c_char,
    mut mode: *const libc::c_char,
    mut p: *const uint8_t,
    mut n: size_t,
) -> libc::c_int {
    let mut fp: *mut FILE = fopen(path, mode);
    let mut rc: libc::c_int = 0;
    if fp.is_null() {
        return -(1 as libc::c_int);
    }
    rc = if fwrite(p as *const libc::c_void, 1 as libc::c_int as libc::c_ulong, n, fp)
        == n
    {
        0 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
    if fclose(fp) != 0 as libc::c_int {
        rc = -(1 as libc::c_int);
    }
    return rc;
}
unsafe extern "C" fn db_alloc_users(mut db: *mut db_t, mut n: uint32_t) -> libc::c_int {
    (*db)
        .users = calloc(
        n.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong,
        ::core::mem::size_of::<user_t>() as libc::c_ulong,
    ) as *mut user_t;
    return if !((*db).users).is_null() { 0 as libc::c_int } else { -(1 as libc::c_int) };
}
unsafe extern "C" fn db_load(mut db: *mut db_t) -> libc::c_int {
    if access(b"enc.db\0" as *const u8 as *const libc::c_char, 0 as libc::c_int)
        != 0 as libc::c_int
    {
        return db_alloc_users(db, 0 as libc::c_int as uint32_t);
    }
    if read_file(
        b"enc.db\0" as *const u8 as *const libc::c_char,
        &mut (*db).raw,
        &mut (*db).raw_len,
    ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    return db_parse(db);
}
unsafe extern "C" fn db_free(mut db: *mut db_t) {
    free((*db).raw as *mut libc::c_void);
    free((*db).users as *mut libc::c_void);
}
unsafe extern "C" fn db_find(
    mut db: *mut db_t,
    mut name: *const libc::c_char,
) -> *mut user_t {
    let mut n: size_t = strlen(name);
    let mut i: uint32_t = 0;
    i = 0 as libc::c_int as uint32_t;
    while i < (*db).num_users {
        if (*((*db).users).offset(i as isize)).name_len as libc::c_ulong == n
            && memcmp(
                (*((*db).users).offset(i as isize)).name as *const libc::c_void,
                name as *const libc::c_void,
                n,
            ) == 0 as libc::c_int
        {
            return &mut *((*db).users).offset(i as isize) as *mut user_t;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as *mut user_t;
}
unsafe extern "C" fn hash_filename(
    mut u: *const user_t,
    mut fname: *const libc::c_char,
    mut out: *mut uint8_t,
) {}
unsafe extern "C" fn has_name_hash(
    mut u: *const user_t,
    mut name_hash: *const uint8_t,
) -> libc::c_int {
    let mut i: uint32_t = 0;
    i = 0 as libc::c_int as uint32_t;
    while i < (*u).num_hashes {
        if memcmp(
            ((*u).name_hashes)
                .offset(
                    (i as size_t).wrapping_mul(32 as libc::c_int as libc::c_ulong)
                        as isize,
                ) as *const libc::c_void,
            name_hash as *const libc::c_void,
            32 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            return 1 as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn find_file(
    mut uf: *mut userfiles_t,
    mut name_hash: *const uint8_t,
) -> *mut ufile_t {
    let mut i: uint32_t = 0;
    i = 0 as libc::c_int as uint32_t;
    while i < (*uf).nfiles {
        if memcmp(
            (*((*uf).files).offset(i as isize)).name_hash as *const libc::c_void,
            name_hash as *const libc::c_void,
            32 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            return &mut *((*uf).files).offset(i as isize) as *mut ufile_t;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as *mut ufile_t;
}
unsafe extern "C" fn free_files(mut uf: *mut userfiles_t) {
    free((*uf).pt as *mut libc::c_void);
    free((*uf).files as *mut libc::c_void);
}
unsafe extern "C" fn set_file(
    mut uf: *mut userfiles_t,
    mut name_hash: *const uint8_t,
    mut data: *const uint8_t,
    mut len: size_t,
) {
    let mut f: *mut ufile_t = find_file(uf, name_hash);
    if f.is_null() {
        let fresh0 = (*uf).nfiles;
        (*uf).nfiles = ((*uf).nfiles).wrapping_add(1);
        f = &mut *((*uf).files).offset(fresh0 as isize) as *mut ufile_t;
        (*f).name_hash = name_hash;
    }
    (*f).data = data;
    (*f).len = len as uint32_t;
}
unsafe extern "C" fn parse_action(mut s: *const libc::c_char) -> action_t {
    if strcmp(s, b"register\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        return ACT_REGISTER;
    }
    if strcmp(s, b"create\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        return ACT_CREATE;
    }
    if strcmp(s, b"read\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        return ACT_READ;
    }
    if strcmp(s, b"write\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        return ACT_WRITE;
    }
    return ACT_NONE;
}
unsafe extern "C" fn parse_args(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut a: *mut args_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    memset(
        a as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<args_t>() as libc::c_ulong,
    );
    i = 1 as libc::c_int;
    while i < argc {
        let mut s: *const libc::c_char = *argv.offset(i as isize);
        let mut slot: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
        let mut act: action_t = ACT_NONE;
        if strcmp(s, b"-u\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
            slot = &mut (*a).user;
        } else if strcmp(s, b"-k\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            slot = &mut (*a).key;
        } else if strcmp(s, b"-f\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            slot = &mut (*a).file;
        } else if strcmp(s, b"-i\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            slot = &mut (*a).in_0;
        } else if strcmp(s, b"-o\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            slot = &mut (*a).out;
        }
        if !slot.is_null() {
            if i + 1 as libc::c_int >= argc {
                return -(1 as libc::c_int);
            }
            i += 1;
            *slot = *argv.offset(i as isize);
        } else if *s.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
            && *s.offset(1 as libc::c_int as isize) as libc::c_int != '\0' as i32
        {
            return -(1 as libc::c_int)
        } else {
            act = parse_action(s);
            if act as libc::c_uint != ACT_NONE as libc::c_int as libc::c_uint
                && (*a).act as libc::c_uint == ACT_NONE as libc::c_int as libc::c_uint
            {
                (*a).act = act;
            } else if ((*a).text).is_null() {
                (*a).text = s;
            } else {
                return -(1 as libc::c_int)
            }
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn check_args(mut a: *const args_t) -> libc::c_int {
    if ((*a).user).is_null() {
        return -(1 as libc::c_int);
    }
    match (*a).act as libc::c_uint {
        1 => {
            return if !((*a).key).is_null() && ((*a).in_0).is_null()
                && ((*a).out).is_null() && ((*a).text).is_null()
            {
                0 as libc::c_int
            } else {
                -(1 as libc::c_int)
            };
        }
        2 => {
            return if !((*a).file).is_null() && ((*a).in_0).is_null()
                && ((*a).out).is_null() && ((*a).text).is_null()
            {
                0 as libc::c_int
            } else {
                -(1 as libc::c_int)
            };
        }
        3 => {
            return if !((*a).key).is_null() && !((*a).file).is_null()
                && ((*a).in_0).is_null() && ((*a).text).is_null()
            {
                0 as libc::c_int
            } else {
                -(1 as libc::c_int)
            };
        }
        4 => {
            return if !((*a).key).is_null() && !((*a).file).is_null()
                && ((*a).out).is_null()
                && ((*a).in_0).is_null() as libc::c_int
                    != ((*a).text).is_null() as libc::c_int
                && (((*a).text).is_null()
                    || *((*a).text).offset(0 as libc::c_int as isize) as libc::c_int
                        != '\0' as i32)
            {
                0 as libc::c_int
            } else {
                -(1 as libc::c_int)
            };
        }
        _ => return -(1 as libc::c_int),
    };
}
unsafe extern "C" fn do_create(mut db: *mut db_t, mut a: *const args_t) -> libc::c_int {
    let mut name_hash: [uint8_t; 32] = [0; 32];
    let mut name_hashes: *mut uint8_t = 0 as *mut uint8_t;
    let mut old: size_t = 0;
    let mut u: *mut user_t = 0 as *mut user_t;
    let mut rc: libc::c_int = 0;
    u = db_find(db, (*a).user);
    if u.is_null() {
        return -(1 as libc::c_int);
    }
    hash_filename(u, (*a).file, name_hash.as_mut_ptr());
    if has_name_hash(u, name_hash.as_mut_ptr()) != 0 {
        return -(1 as libc::c_int);
    }
    old = ((*u).num_hashes as size_t).wrapping_mul(32 as libc::c_int as libc::c_ulong);
    name_hashes = malloc(old.wrapping_add(32 as libc::c_int as libc::c_ulong))
        as *mut uint8_t;
    if name_hashes.is_null() {
        return -(1 as libc::c_int);
    }
    memcpy(
        name_hashes as *mut libc::c_void,
        (*u).name_hashes as *const libc::c_void,
        old,
    );
    memcpy(
        name_hashes.offset(old as isize) as *mut libc::c_void,
        name_hash.as_mut_ptr() as *const libc::c_void,
        32 as libc::c_int as libc::c_ulong,
    );
    (*u).name_hashes = name_hashes;
    (*u).num_hashes = ((*u).num_hashes).wrapping_add(1);
    (*u).num_hashes;
    rc = db_save(db);
    if rc == 0 as libc::c_int {
        printf(
            b"File '%s' has been created for user '%s'\n\0" as *const u8
                as *const libc::c_char,
            (*a).file,
            (*a).user,
        );
    }
    free(name_hashes as *mut libc::c_void);
    return rc;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut a: args_t = args_t {
        user: 0 as *const libc::c_char,
        key: 0 as *const libc::c_char,
        file: 0 as *const libc::c_char,
        in_0: 0 as *const libc::c_char,
        out: 0 as *const libc::c_char,
        text: 0 as *const libc::c_char,
        act: ACT_NONE,
    };
    let mut db: db_t = {
        let mut init = db_t {
            raw: 0 as *mut uint8_t,
            raw_len: 0,
            users: 0 as *mut user_t,
            num_users: 0,
        };
        init
    };
    let mut rc: libc::c_int = -(1 as libc::c_int);
    if sodium_init() >= 0 as libc::c_int
        && parse_args(argc, argv, &mut a) == 0 as libc::c_int
        && check_args(&mut a) == 0 as libc::c_int && db_load(&mut db) == 0 as libc::c_int
    {
        match a.act as libc::c_uint {
            1 => {
                rc = do_register(&mut db, &mut a);
            }
            2 => {
                rc = do_create(&mut db, &mut a);
            }
            3 => {
                rc = do_read(&mut db, &mut a);
            }
            4 => {
                rc = do_write(&mut db, &mut a);
            }
            _ => {}
        }
    }
    db_free(&mut db);
    if rc != 0 as libc::c_int {
        printf(b"invalid\n\0" as *const u8 as *const libc::c_char);
        return 255 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
