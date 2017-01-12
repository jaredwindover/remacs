use std::os::raw::c_char;
use std::ptr;

extern crate libc;

use lisp::{XTYPE, LispObject, LispType, LispSubr, Qnil,
           VectorLikeHeader, PvecType, PSEUDOVECTOR_AREA_BITS};

extern "C" {
    static Qt: LispObject;
}

/// Is this LispObject a symbol?
#[allow(non_snake_case)]
pub fn SYMBOLP(a: LispObject) -> bool {
    XTYPE(a) == LispType::Lisp_Symbol
}

#[test]
fn test_symbolp() {
    assert!(SYMBOLP(Qnil));
}

fn Fsymbolp(object: LispObject) -> LispObject {
    if SYMBOLP(object) {
        unsafe { Qt }
    } else {
        Qnil
    }
}

defun!("symbolp", Fsymbolp, Ssymbolp, 1, 1, ptr::null(), "Return t if OBJECT is a symbol.

(fn OBJECT)");
