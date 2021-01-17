//! Module metadata
//!
//! This data is returned by get_module_metadata()
//! which is generated by [extendr_module!].
use crate::*;
use std::io::Write;

/// Metadata function argument.
#[derive(Debug, PartialEq)]
pub struct Arg {
    pub name: &'static str,
    pub arg_type: &'static str,
}

/// Metadata function.
#[derive(Debug, PartialEq)]
pub struct Func {
    pub doc: &'static str,
    pub name: &'static str,
    pub args: Vec<Arg>,
    pub return_type: &'static str,
    pub func_ptr: *const u8,
    pub hidden: bool,
}

/// Metadata Impl.
#[derive(Debug, PartialEq)]
pub struct Impl {
    pub doc: &'static str,
    pub name: &'static str,
    pub methods: Vec<Func>,
}

/// Module metadata.
#[derive(Debug, PartialEq)]
pub struct Metadata {
    pub functions: Vec<Func>,
    pub impls: Vec<Impl>,
}

impl From<Arg> for Robj {
    fn from(val: Arg) -> Self {
        let res: Robj = List(&[r!(val.name), r!(val.arg_type)]).into();
        res.set_attrib(names_symbol(), r!(["name", "arg_type"]));
        res
    }
}

impl From<Func> for Robj {
    fn from(val: Func) -> Self {
        let res: Robj = List(&[
            r!(val.doc),
            r!(val.name),
            r!(List(val.args)),
            r!(val.return_type),
            r!(val.hidden),
        ])
        .into();
        res.set_attrib(
            names_symbol(),
            r!(["doc", "name", "args", "return.type", "hidden"]),
        );
        res
    }
}

impl From<Impl> for Robj {
    fn from(val: Impl) -> Self {
        let res: Robj = List(&[r!(val.doc), r!(val.name), r!(List(val.methods))]).into();
        res.set_attrib(names_symbol(), r!(["doc", "name", "methods"]));
        res
    }
}

impl From<Metadata> for Robj {
    fn from(val: Metadata) -> Self {
        let res: Robj = List(&[r!(List(val.functions)), r!(List(val.impls))]).into();
        res.set_attrib(names_symbol(), r!(["functions", "impls"]));
        res
    }
}

fn write_r_wrapper(w: &mut Vec<u8>, func: &Func, use_symbols: bool) -> std::io::Result<()> {
    if func.hidden {
        return Ok(());
    }

    let args = func.args.iter().map(|arg| arg.name).collect::<Vec<_>>().join(", ");

    if !func.doc.is_empty() {
        write!(w, "#'")?;
        for c in func.doc.chars() {
            if c == '\n' {
                write!(w, "\n#'")?;
            } else {
                write!(w, "{}", c)?;
            }
        }
        writeln!(w, "")?;
    }
    
    write!(w, "{} <- function({}) .Call(", func.name, args)?;

    if use_symbols {
        write!(w, "wrap__{}", func.name)?;
    } else {
        write!(w, "\"wrap__{}\"", func.name)?;
    }

    if func.args.is_empty() {
        writeln!(w, ")")?;
    } else {
        writeln!(w, ", {})", args)?;
    }
    Ok(())
}

impl Metadata {
    pub fn make_r_wrappers(&self, use_symbols: bool) -> String {
        let mut res = Vec::new();
        for func in &self.functions {
            write_r_wrapper(&mut res, func, use_symbols).unwrap();
        }

        for imp in &self.impls {
            for func in &imp.methods {
                write_r_wrapper(&mut res, func, use_symbols).unwrap();
            }
        }
        unsafe { String::from_utf8_unchecked(res) }
    }
}
