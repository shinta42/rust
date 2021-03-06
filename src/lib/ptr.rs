/*
Module: ptr

Unsafe pointer utility functions
*/
native "rust-intrinsic" mod rusti {
    fn addr_of<T>(val: T) -> *T;
    fn ptr_offset<T>(ptr: *T, count: uint) -> *T;
}

/*
Function: addr_of

Get an unsafe pointer to a value
*/
fn addr_of<T>(val: T) -> *T { ret rusti::addr_of(val); }

/*
Function: mut_addr_of

Get an unsafe mutable pointer to a value
*/
fn mut_addr_of<T>(val: T) -> *mutable T unsafe {
    ret unsafe::reinterpret_cast(rusti::addr_of(val));
}

/*
Function: offset

Calculate the offset from a pointer
*/
fn offset<T>(ptr: *T, count: uint) -> *T {
    ret rusti::ptr_offset(ptr, count);
}

/*
Function: null

Create an unsafe null pointer
*/
fn null<T>() -> *T unsafe { ret unsafe::reinterpret_cast(0u); }
