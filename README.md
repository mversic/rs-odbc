# RS-ODBC

[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://github.com/mversic/rs-odbc/blob/master/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/rs-odbc.svg)](https://crates.io/crates/rs-odbc)

Rust implementation of the ODBC API that looks and feels like ODBC but is safe

## Description

Main design goal of this crate is that the **exposed API should look as close as possible to the original ODBC API**
while providing type safety wherever possible. This crate prevents most of the safety issues inherent to C code
and moves most of the application errors to compile time. ODBC state transitions FSM is implemented inside Rust's
type system so that many of the invalid handle errors are prevented as compile errors.

## Why this crate

### 1. Known API
If you have already worked with the ODBC API you will feel at home using this crate, i.e. **you don't have
to learn yet another API**. With this crate you are getting a well known, highly used and standardized API.
The level of abstraction over the original ODBC API is minimal so that **you can use the original ODBC
documentation**. Translating existing ODBC applications or examples from C to Rust is very straightforward.

### 2. Safe API
For most applications **you will never have to resort to using raw pointers**. Crates which expose custom high-level
API wrappers around ODBC will most likely force you to fall back to using the raw API when you are required
to use ODBC features that are not expressible through the API they provide. This will introduce
unnecessary safety risks for your application unless those crates are built on top of this crate.

### 3. Complete API
**This crate is designed to be fully ODBC compliant** so there should be no low-level ODBC feature that cannot be
expressed through this crate. However, it is possible that a particular feature may not have been implemented
yet. If you notice that a feature is missing, you are encouraged to open an issue requiring the feature.

# Installation

To be able to use this library you have to have ODBC Driver Manager installed and configured on your host OS.
This library dynamically links against the `odbc32.dll` on Windows or against `libodbc.so`(unixODBC) on Linux
and OS-X. To enable static linking of native libraries use the cargo `static` feature.

# Cargo features

## static

Enables static linking of native libraries. If static linking is enabled user must define `RS_ODBC_LINK_SEARCH`
environment variable which contains path to static libraries this crate will link against. For unixODBC, user
should provide both `libodbc.a` and `libltdl.a` under this path. **Static linking is not supported for Windows**.

# API differences

1. ODBC functions are implemented as methods or associated functions on handles:

<table>
<tr>
<th>C ODBC example</th>
<th>Rust ODBC example</th>
</tr>
<tr>
<td>

```c
SQLHSTMT hstmt = SQL_NULL_HSTMT;
int ret1 = SQLAllocHandle(
    SQL_HANDLE_STMT,
    hdbc,
    &hstmt
);
if (SQL_SUCCEEDED(ret1)) {
    int ret2 = SQLCancelHandle(
        SQL_HANDLE_STMT,
        hstmt
    );
}
```
</td>
<td>

```rust
use rs_odbc::handle::HSTMT;

let hstmt = HSTMT::alloc_handle(&hdbc).unwrap();
let ret = hstmt.cancel_handle();
```

</td>
</tr>
</table>

2. Most ODBC handle methods return `sqlreturn::RETURN`, as specified by ODBC. Operations that
change a handle's state return a dedicated outcome enum containing the correctly typed handle
(for example, `DriverConnectResult`). Returning handles makes it possible to implement the ODBC
state transition FSM in Rust's type system.

3. ODBC functions which take pointer and it's length take reference to a slice instead. Slice references
prevent the possibility of the application writer to write/read past the end of the allocation unit.

<table>
<tr>
<th>C ODBC example</th>
<th>Rust ODBC example</th>
</tr>
<tr>
<td>

```c
SQLCHAR catalog_name[] = "rs_odbc";
int ret = SQLSetConnAttr(
    hdbc,
    SQL_ATTR_CURRENT_CATALOG,
    catalog_name,
    SQL_NTS,
);
```
</td>
<td>

```rust
use rs_odbc::{attr::CURRENT_CATALOG, data::CHAR};

let catalog_name = "rs_odbc";
let ret = hdbc.set_attr::<CURRENT_CATALOG, CHAR>(catalog_name.as_ref());
```

</td>
</tr>
</table>

4. ODBC version is defined at the point when environment handle is allocated. Usually, this
should be the first step in your ODBC application but in Rust it is handled by the type system

<table>
<tr>
<th>C ODBC example</th>
<th>Rust ODBC example</th>
</tr>
<tr>
<td>

```c
SQLHENV henv = SQL_NULL_HENV;
int ret1 = SQLAllocHandle(
    SQL_HANDLE_ENV,
    &SQL_NULL_HANDLE,
    &henv
);
if (SQL_SUCCEEDED(ret1)) {
    int ret2 = SQLSetEnvAttr(
        henv,
        SQL_ATTR_ODBC_VERSION,
        SQL_OV_ODBC3_80,
        0
    );
}
```
</td>
<td>

```rust
use rs_odbc::{env::OV_ODBC3_80, handle::HENV};

let env = HENV::<OV_ODBC3_80>::alloc_handle().unwrap();
```

</td>
</tr>
</table>

5. Disconnecting and freeing handles is done automatically at the end of scope

# Uninitialized variables

When a sensible fallback exists, prefer `MaybeUninit::new(value)` so as to avoid the UB.

# Thread safety

Environment handles are `Send + Sync`, and connection handles are `Send` but not `Sync`. Statement and descriptor handles must remain on the thread where they are used.
Obviously, to cancel a function running on a connection or statement handle on another thread one must be able to share a handle reference across threads.
Since the operation of **canceling is defined by the ODBC standard to always be a thread safe operation**, for this specific scenario, from your original handle,
you can derive a handle that implements the `Sync` trait such as `WeakSQLHSTMT` or `RefSQLHSTMT`. Handles prefixed with `Ref` are allocated from a reference
to your original handle, while ones that are prefixed `Weak` are allocated from your original handle wrapped in an `Arc`.

```rust
// TODO: Add code example
```

If there is a use-case where you would like to be able to share handles other than `SQLHENV` among threads, please open an issue describing your use-case.


# Unsafe API

There are cases where it's not possible to ensure safety through the type system.  In these rare cases you can allocate `UnsafeSQLHSTMT` and `UnsafeSQLHDESC`
which implement additional unsafe API which makes some of the statement functions unsafe

```rust
// TODO:
```

# Testing

Integration tests use dockerized environment which has database and ODBC driver already set up.

Testing environment can be set up with `docker compose up -d`<br/>
Tests are executed with `docker exec -t rs-odbc sh -lc 'cargo test'`

* use `RUSTFLAGS=-Awarnings` to silence compiler warnings which make compile tests fail
