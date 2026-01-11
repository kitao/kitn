use super::pocketpy_sys::*;
use std::ffi::{CStr, CString};
use std::ptr;
use std::os::raw::c_int;

pub struct PocketPy;

impl PocketPy {
    pub fn new() -> Self {
        unsafe {
            py_initialize();
            Self
        }
    }

    pub fn exec(&mut self, source: &str) -> bool {
        let source_c = CString::new(source).unwrap();
        let filename_c = CString::new("main.py").unwrap();
        unsafe {
            py_exec(
                source_c.as_ptr(),
                filename_c.as_ptr(),
                py_CompileMode_EXEC_MODE,
                ptr::null_mut()
            )
        }
    }

    pub fn bind_print(&mut self) {
        unsafe {
            let name = CString::new("print").unwrap();
            let sig = CString::new("print(*args)").unwrap();
            // In new API, py_bind takes object (usually module), signature, and function.
            // We want to bind to builtins probably? Or global.
            // Let's try binding to current module or global.
            // If obj is NULL, it might bind to builtins or global?
            // Bindings signature: py_bind(obj: py_Ref, sig: *const c_char, f: py_CFunction)

            // To bind global function 'print', we probably need the global context or builtins module.
            // py_getbuiltin(name) returns item ref.
            // Let's try binding to the result of py_getmodule("__main__") or just use the newvm context implicit?
            // Actually, py_newnativefunc might be needed if we want to return a function object and assign it.
            // But py_bind seems designed to attach methods.

            // Wait, PocketPy C interface usually executes in a context.
            // If we want 'print' to be available globally, we should add it to builtins.
            // But 'print' is already a builtin in Python. We want to OVERRIDE it or adding a new one.
            // Overriding print in builtins:

            // Let's assume we can bind to the builtins module.
            // Is there py_getmodule("builtins")?

            // For simplicity, let's just define a global function 'godot_print' first to test.
            // Using `py_bind` on the main module.
            let main_mod = py_getmodule(CString::new("__main__").unwrap().as_ptr());
            py_bind(main_mod, sig.as_ptr(), Some(Self::python_print));
        }
    }

    extern "C" fn python_print(argc: c_int, _argv: py_StackRef) -> bool {
        unsafe {
             godot::prelude::godot_print!("[PocketPy] print() called with {} args", argc);

             // Argument printing is currently unstable due to assertion failures in py_tostr/py_tosv.
             // TODO: Fix argument string conversion.
             /*
            let mut buf = String::new();
            for i in 0..argc {
                let elem = _argv.offset(i as isize);
                if py_repr(elem) {
                     let s_obj = py_peek(-1);
                     // py_repr guarantees a string at stack top if it returns true.
                     // The previous crash was py_totype(s_obj) -> self->type == tp_type check failing?
                     // Or py_tostr(s_obj) -> self->type == tp_str check failing?
                     // If py_repr succeeded, stack top should be a string.

                     // Let's rely on py_tosv directly.
                     let sv = py_tosv(s_obj);
                     if sv.data.is_null() {
                         buf.push_str("<null sv>");
                     } else {
                         let s = CStr::from_ptr(sv.data).to_string_lossy();
                         if i > 0 { buf.push(' '); }
                         buf.push_str(&s);
                     }
                     py_pop();
                } else {
                    buf.push_str("<repr failed>");
                }
            }
            godot::prelude::godot_print!("[PocketPy] {}", buf);
            */
            py_newnone(py_retval());
            true
        }
    }
}

impl Drop for PocketPy {
    fn drop(&mut self) {
        unsafe {
            py_finalize();
        }
    }
}
