#[macro_use]
extern crate bitflags;
extern crate libc;
extern crate time;
extern crate zstd;

#[macro_use]
mod macros;

pub mod error;
mod redis;

use std::str;
use error::CellError;
use redis::Command;
use redis::raw;
use libc::{c_int, c_long, c_longlong, size_t};
use std::error::Error;
use std::iter;
use std::ptr;
use std::string;

const MODULE_NAME: &str = "redis-cell";
const MODULE_VERSION: c_int = 1;

// ZstdSetCommand provides GCRA rate limiting as a command in Redis.
struct ZstdGetCommand {}

impl Command for ZstdGetCommand {
    // Should return the name of the command to be registered.
    fn name(&self) -> &'static str {
        "zstd.zget"
    }

    // Run the command.
    fn run(&self, r: redis::Redis, args: &[&str]) -> Result<(), CellError> {
        if args.len() != 2 {
            return Err(error!(
                "Usage: {} <key> <period> [<quantity>]",
                self.name()
            ));
        }

        let key = args[1];
        let restr = r.open_key(&key).read_zstd_decompress_str().unwrap().unwrap();
        r.reply_string(&restr);

        Ok(())
    }
    fn str_flags(&self) -> &'static str {
        "write"
    }
}

struct ZstdSetCommand {}
impl Command for ZstdSetCommand {
    // Should return the name of the command to be registered.
    fn name(&self) -> &'static str {
        "zstd.zset"
    }

    // Run the command.
    fn run(&self, r: redis::Redis, args: &[&str]) -> Result<(), CellError> {
        if args.len() != 3 {
            return Err(error!(
                "Usage: {} <key> <period> [<quantity>]",
                self.name()
            ));
        }

        let key = args[1];
        let val = args[2];

        r.open_key_writable(key).write_zstd_comp(&val).unwrap();
        r.reply_string(&val);

        Ok(())
    }

    fn str_flags(&self) -> &'static str {
        "write"
    }
}
#[allow(non_snake_case)]
#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn ZstdGet_RedisCommand(
    ctx: *mut raw::RedisModuleCtx,
    argv: *mut *mut raw::RedisModuleString,
    argc: c_int,
) -> raw::Status {
    Command::harness(&ZstdGetCommand {}, ctx, argv, argc)
}

#[allow(non_snake_case)]
#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn ZstdSet_RedisCommand(
    ctx: *mut raw::RedisModuleCtx,
    argv: *mut *mut raw::RedisModuleString,
    argc: c_int,
) -> raw::Status {
    Command::harness(&ZstdSetCommand {}, ctx, argv, argc)
}

#[allow(non_snake_case)]
#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn RedisModule_OnLoad(
    ctx: *mut raw::RedisModuleCtx,
    argv: *mut *mut raw::RedisModuleString,
    argc: c_int,
) -> raw::Status {
    if raw::init(
        ctx,
        format!("{}\0", MODULE_NAME).as_ptr(),
        MODULE_VERSION,
        raw::REDISMODULE_APIVER_1,
    ) == raw::Status::Err
    {
        return raw::Status::Err;
    }

    let get_command = ZstdGetCommand {};
    let set_command = ZstdSetCommand {};
    if raw::create_command(
        ctx,
        format!("{}\0", get_command.name()).as_ptr(),
        Some(ZstdGet_RedisCommand),
        format!("{}\0", get_command.str_flags()).as_ptr(),
        0,
        0,
        0,
    ) == raw::Status::Err
    {
        return raw::Status::Err;
    }

    if raw::create_command(
        ctx,
        format!("{}\0", set_command.name()).as_ptr(),
        Some(ZstdSet_RedisCommand),
        format!("{}\0", set_command.str_flags()).as_ptr(),
        0,
        0,
        0,
    ) == raw::Status::Err
    {
        return raw::Status::Err;
    }

    raw::Status::Ok
}
