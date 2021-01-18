initSidebarItems({"attr":[["extendr","Generate bindings for a single function."]],"constant":[["FALSE","FALSE value eg. `r!(FALSE)`"],["NA_INTEGER","NA value for integers eg. `r!(NA_INTEGER)`"],["NA_LOGICAL","NA value for logical. `r!(NA_LOGICAL)`"],["NA_REAL","NA value for real values eg. `r!(NA_REAL)`"],["NA_STRING","NA value for strings. `r!(NA_STRING)`"],["NULL","NULL value eg. `r!(NULL)`"],["TRUE","TRUE value eg. `r!(TRUE)`"]],"enum":[["Error",""],["Robj","Wrapper for an R S-expression pointer (SEXP)."]],"fn":[["base_env","The base environment; formerly R_NilValue"],["base_namespace","The namespace for base."],["base_symbol","\"base\""],["blank_scalar_string","\"\" as a STRSXP"],["blank_string","\"\" as a CHARSXP"],["brace_symbol","\"{\""],["bracket_2_symbol","\"[[\""],["bracket_symbol","\"[\""],["catch_r_error","Wrap an R function such as Rf_findFunction and convert errors and panics into results."],["class_symbol","\"class\""],["current_env","The current interpreter environment."],["device_symbol","\".Device\""],["dim_symbol","\"dim\""],["dimnames_symbol","\"dimnames\""],["dollar_symbol","\"$\""],["dot_defined","\".defined\""],["dot_method","\".Method\""],["dot_package_name","\"packageName\""],["dot_target","\".target\""],["dots_symbol","\"...\""],["double_colon_symbol","\"::\""],["empty_env","An empty environment at the root of the environment tree"],["find_namespace","Find a namespace by name."],["global_env","The \"global\" environment"],["global_function","Get a global function from global_env() and ancestors."],["global_var","Get a global variable from global_env() and ancestors. If the result is a promise, evaulate the promise."],["lastvalue_symbol","\".Last.value\""],["levels_symbol","\"levels\""],["local_var","Get a local variable from current_env() and ancestors."],["missing_arg","Missing argument marker"],["mode_symbol","\"mode\""],["na_rm_symbol","\"na.rm\""],["na_str","Special \"NA\" string that represents null strings."],["na_string","NA_STRING as a CHARSXP"],["name_symbol","\"name\""],["names_symbol","\"names\""],["namespace_env_symbol","NAMESPACE_.\""],["namespace_registry","For registered namespaces."],["new_env","Create a new, empty environment parented on global_env()"],["new_env_with_capacity","Create a new, empty environment parented on global_env() with a reserved size."],["nil_value","The nil object"],["package_symbol","\"package\""],["previous_symbol","\"previous\""],["quote_symbol","\"quote\""],["row_names_symbol","\"row.names\""],["seeds_symbol","\".Random.seed\""],["single_threaded","Run a function single threaded. Note: This will fail badly if the called function panics or calls RF_error."],["sort_list_symbol","\"sort.list\""],["source_symbol","\"source\""],["spec_symbol","\"spec\""],["srcref","Current srcref, for debuggers"],["test","Extendr test harness."],["this_thread_id",""],["throw_r_error",""],["triple_colon_symbol","\":::\""],["tsp_symbol","\"tsp\""],["unbound_value","Unbound marker"]],"macro":[["R","Call inline R source code from Rust."],["c","Concatenation operator."],["call","The call! macro calls an R function with Rust parameters. Equivalent to `lang!(sym, params).eval()` This returns a Rust Result."],["data_frame","Create a dataframe."],["extendr_module","Define a module and export symbols to R Example:"],["factor","Create a factor."],["global","Get a global variable."],["lang","A macro for constructing R langage objects."],["list","Create a list."],["r","Convert a rust expression to an R object."],["read_table","Read a CSV file."],["rep","Create a vector with repeating elements."],["reprint","Print via the R error stream."],["reprintln","Print with a newline via the R output stream."],["rprint","Print via the R output stream."],["rprintln","Print with a newline via the R output stream."],["sym","The sym! macro install symbols. You should cache your symbols in variables as generating them is costly."],["test","Macro for running tests."],["var","Get a local variable from the calling function or a global variable if no such variable exists."]],"struct":[["Bool","Bool is a wrapper for i32 in the context of an R's tristate boolean. It can be TRUE, FALSE or NA_LOGICAL."],["Character","Wrapper for creating character objects. These are used only as the contents of a character vector."],["Env","Wrapper for creating environments."],["EnvIter","Iterator over the names and values of an environment"],["Expr","Wrapper for creating expression objects."],["Func","Wrapper for creating functions (CLOSSXP)."],["Lang","Wrapper for creating language objects."],["List","Wrapper for creating list (VECSXP) objects."],["ListIter",""],["Pairlist","Wrapper for creating pair list (LISTSXP) objects."],["PairlistIter","Iterator over the objects in a vector or string."],["PairlistTagIter","Iterator over pairlist tag names."],["Primitive","Wrapper for creating and reading Primitive functions."],["Promise",""],["RArray","Wrapper for creating and using matrices and arrays."],["Raw","Wrapper for creating raw (byte) objects."],["StrIter","Iterator over strings or string factors."]],"trait":[["AsTypedSlice","Generic access to typed slices in an Robj."],["FromRobj","Trait used for incomming parameter conversion."],["IsNA","Return true if this primitive is NA."],["RobjItertools","Extensions to iterators for R objects including [RobjItertools::collect_robj()]."],["SymPair",""],["ToVectorValue","`ToVectorValue` is a trait that allows many different types to be converted to vectors. It is used as a type parameter to `collect_robj()`."]],"type":[["IntegerIter","Iterator over primitives in integer objects."],["LogicalIter","Iterator over primitives in logical objects."],["NamedListIter","Iterator over name-value pairs in lists."],["RColumn",""],["RMatrix",""],["RMatrix3D",""],["RealIter","Iterator over primitives in real objects."],["Result",""]]});