use syn::Pat;

use syn::FnArg;

use syn::Signature;

use syn;

use std::collections::HashMap;

#[derive(Debug)]
pub struct CallFn {
    pub impl_dependencies: String,
    pub fn_name: Vec<String>,
    pub sig: Vec<String>,
    pub args: Vec<String>,
}

pub fn parse_file(mut content: &str) -> HashMap<String, CallFn> {
    // Strip the BOM if it is present
    const BOM: &str = "\u{feff}";
    if content.starts_with(BOM) {
        content = &content[BOM.len()..];
    }

    const FLAG: &str = "/// impl_trait:";

    let mut trait_sig_pool = HashMap::new();

    for mut line in content.split('\n') {
        if line.starts_with(FLAG) {
            line = &line[FLAG.len()..];
            let mut iter = line.split('|');
            let impl_ = iter.next().unwrap_or("");
            let trait_ = iter.next().unwrap();

            let mut call_fn = CallFn {
                impl_dependencies: impl_.trim().to_owned(),
                fn_name: Vec::new(),
                sig: Vec::new(),
                args: Vec::new(),
            };
            for item in iter {
                let fn_ = item.replace("\\n", "\n");

                let sig = syn::parse_str::<Signature>(&fn_)
                    .map_err(|e| panic!("Invalid {}: {}", e, fn_))
                    .unwrap();

                let mut args = format!("");
                for sig_input in sig.inputs.iter() {
                    if let FnArg::Typed(ref pat_type) = sig_input {
                        if let Pat::Ident(ref pat_ident) = *pat_type.pat {
                            args += format!("{},", pat_ident.ident).as_str();
                        }
                    }
                }
                call_fn
                    .fn_name
                    .push(sig.ident.to_string().trim().to_owned());
                call_fn.sig.push(fn_.trim().to_owned());
                call_fn.args.push(args);
            }
            trait_sig_pool.insert(trait_.trim().to_owned(), call_fn);
        }
    }
    trait_sig_pool
}
