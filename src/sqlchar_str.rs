use crate::{AsMutRawSlice, AsRawSlice, SQLCHAR, SQLSMALLINT, AsMutSQLPOINTER, SQLPOINTER};

#[derive(Debug)]
pub struct SQLCHARString<T> {
    inner: Vec<SQLCHAR>,
    size: std::marker::PhantomData<T>,
}

impl SQLCHARString<SQLSMALLINT> {
    pub fn new(input: String) -> Self {
        let mut inner = input.into_bytes();
        // TODO: Just to be on the safe side
        inner.push('\0' as SQLCHAR);

        Self {
            inner,
            size: std::marker::PhantomData,
        }
    }

    pub fn with_capacity(cap: usize) -> Self {
        let mut inner = Vec::with_capacity(cap);
        // TODO: Just to be on the safe side
        inner.push('\0' as SQLCHAR);

        Self {
            inner,
            size: std::marker::PhantomData,
        }
    }

    pub fn len(&self) -> SQLSMALLINT {
        self.inner.len() as SQLSMALLINT
    }
}

impl<T: Copy> SQLCHARString<T> {
    pub unsafe fn assume_init(mut self) -> SQLCHARString<T> {
        let ptr = self.inner.as_mut_ptr();

        let mut offset: usize = 0;
        while offset < self.inner.capacity() && *(ptr.offset(offset as isize)) != '\0' as SQLCHAR {
            offset += 1;
        }

        self.inner.set_len(offset + 1);
        self
    }
}

unsafe impl AsRawSlice<SQLCHAR, SQLSMALLINT> for SQLCHARString<SQLSMALLINT> {
    fn as_raw_slice(&self) -> (*const SQLCHAR, SQLSMALLINT) {
        (
            self.inner.as_ptr().cast(),
            self.inner.capacity() as SQLSMALLINT,
        )
    }
}
unsafe impl AsMutRawSlice<SQLCHAR, SQLSMALLINT> for SQLCHARString<SQLSMALLINT> {
    fn as_mut_raw_slice(&mut self) -> (*mut SQLCHAR, SQLSMALLINT) {
        (
            self.inner.as_mut_ptr().cast(),
            self.inner.capacity() as SQLSMALLINT,
        )
    }
}
unsafe impl<LEN> AsMutSQLPOINTER for SQLCHARString<LEN> {
    fn as_mut_SQLPOINTER(&mut self) -> SQLPOINTER {
        unimplemented!()
    }
}
