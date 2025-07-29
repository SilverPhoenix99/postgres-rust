
pub fn fn_name_of_val(f: impl Fn() + 'static) -> &'static str {

    // Rust doesn't guarantee what the format of `type_name_of_val()` will be,
    // so we'll attempt to get the slice safely, assuming it might not match it.
    let mut name = type_name_of_val(&f);

    // The current format should be something like: <crate>(::<module>)?::<fn>(::{{closure}})*
    // We'll just try to remove the last bit.

    while name.ends_with("::{{closure}}") {
        name = &name[..name.len() - "::{{closure}}".len()]
    }

    name
}

/// Qualified function name
#[macro_export]
macro_rules! qual_fn_name {
    () => {{
        static FN_NAME: std::sync::OnceLock<&str> = std::sync::OnceLock::new();
        *(FN_NAME.get_or_init(|| $crate::fn_name_of_val(||{})))
    }};
}

#[macro_export]
macro_rules! fn_info {
    () => {{
        static FN_INFO: std::sync::OnceLock<$crate::FnInfo> = std::sync::OnceLock::new();
        FN_INFO.get_or_init(|| $crate::FnInfo::new(file!(), line!(), $crate::qual_fn_name!()))
    }};

    ($fn_name:expr) => {{
        const FN_INFO: $crate::FnInfo = $crate::FnInfo::new(file!(), line!(), $fn_name);
        &FN_INFO
    }};
}

pub mod ascii;
pub mod guc;
pub mod mphf;
pub mod wchar;
mod reexport;

reexport! { pub
    char_buffer,
    concealable,
    fn_info,
    impl_from,
    location,
    named,
    non_negative,
    number_radix,
    str,
}

pub type Oid = u32;
pub type QualifiedName = Vec<Str>;

pub const NAMEDATALEN: usize = 64;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qual_fn_name() {
        let fn_name = module_path!().to_owned() + "::test_qual_fn_name";
        assert_eq!(fn_name, qual_fn_name!());
    }

    #[test]
    fn test_fn_info() {
        let fn_info = fn_info!();
        let fn_name = module_path!().to_owned() + "::test_fn_info";
        assert_eq!(fn_name, fn_info.function());
    }

    #[test]
    fn test_fn_info_inside_closure() {

        #[allow(clippy::redundant_closure_call)]
        let fn_info = (|| { fn_info!() })();

        let fn_name = module_path!().to_owned() + "::test_fn_info_inside_closure";
        assert_eq!(fn_name, fn_info.function());
    }
}

use core::any::type_name_of_val;
