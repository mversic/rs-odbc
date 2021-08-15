# RS-ODBC

[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://github.com/mversic/rs-odbc/blob/master/LICENSE)
![Build](https://github.com/mversic/rs-odbc/actions/workflows/odbc-ci.yml/badge.svg)

Rust implementation of the ODBC API that looks and feels like ODBC

## Description

Main design goal driving the development of this crate is that the exposed API should look as close
as possible to the original API while providing type safety wherever possible. The crate prevents most
of the safety issues inherent to C code and moves most of the application errors to compile time.
ODBC state transitions are modeled as a Rust type system FSM which makes many of the
invalid handle errors compile errors.

## Why this crate

### Known API
If you have already worked with the ODBC API you will feel at home using this crate, **you
don't have to learn another API**. With this crate you are getting a well known, highly used and standardized API.
The level of abstraction over the original ODBC API is minimal and you can basically use the original
ODBC documentation. Translating existing ODBC examples from C to Rust is very straight forward
with minimal differences as you can see in the following examples

### Safe API
For most of the applications **you will never have to resort to using raw pointers**. Other crates
that expose custom APIs will usually force you to fall back to the raw API when you are required
to use ODBC features that are not expressible through the API they provide. This will introduce
unnecessary safety risks for your application unless those crates are built on top of this crate.

### Complete API
**This crate aims to be fully ODBC compliant** which means there should be no low level
ODBC feature that can't be expressed through this crate. However, it is possible that a particular
feature may not have been implemented yet. If you notice that a feature is missing, you are encouraged
to open an issue requiring the feature.

# Installation
TODO:

# API differences
TODO:

# Uninitialized variables

At most places you can use initialized variables or variables wrapped into `MaybeUninit`
TODO:

# Thread safety

All handles are `Send`, however, at the moment, only SQLHENV is `Sync` as there doesn't seem to be much use in sharing other handles among threads.
If there is a use-case where you would like to be able to share other handles among threads, please open an issue describing your use-case.
TODO: Talk about SQLCancel when cancelling statements

# Unsafe API

There are cases where it's not possible to ensure safety through the type system
TODO:

# Testing

Integration tests use dockerized environment which has database and ODBC driver already set up.

Testing environment can be set up with `docker-compose up -d`<br/>
Tests are executed with `docker exec -t rs-odbc sh -lc 'cargo test'`

* use `RUSTFLAGS=-Awarnings` to silence compiler warnings which make compile tests fail
