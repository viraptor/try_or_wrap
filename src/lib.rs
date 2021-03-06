/// Helper macro for handling multiple result types easily. The result
/// is similar to using `try!`, but instead passing the `Err` through
/// as is, it will be wrapped in a given enum. This allows to simplify
/// handling multiple errors in one function.
///
/// # Example
///
/// ```
/// enum ReadingError {
///     Io(IoError),
///     Parse(ParseError),
/// }
/// fn io_and_parsing() -> Result<(), ReadingError> {
///     let bytes = try_or_wrap!(do_io(), ReadingError::Io);
///     let data = try_or_wrap!(do_parsing(), ReadingError::Parse);
/// }
/// ```
#[macro_export]
macro_rules! try_or_wrap {
    ($expr:expr, $wrap_error:path) => ({
        match $expr {
            Ok(val) => val,
            Err(err) => return Err($wrap_error(err))
        }
    })
}

#[cfg(test)]
#[derive(PartialEq,Debug)]
enum ErrorTypeA {
    Fail,
}

#[cfg(test)]
#[derive(PartialEq,Debug)]
enum ErrorTypeB {
    Fail,
}

#[cfg(test)]
#[derive(PartialEq,Debug)]
enum ErrorWrapper {
    WrapA(ErrorTypeA),
    WrapB(ErrorTypeB),
}

#[cfg(test)]
fn do_work_a(success: bool) -> Result<(), ErrorTypeA> {
    match success {
        true => Ok(()),
        false => Err(ErrorTypeA::Fail),
    }
}

#[cfg(test)]
fn do_work_b(success: bool) -> Result<(), ErrorTypeB> {
    match success {
        true => Ok(()),
        false => Err(ErrorTypeB::Fail),
    }
}

#[cfg(test)]
fn try_both(success_a: bool, success_b: bool) -> Result<(), ErrorWrapper> {
    try_or_wrap!(do_work_a(success_a), ErrorWrapper::WrapA);
    try_or_wrap!(do_work_b(success_b), ErrorWrapper::WrapB);
    Ok(())
}

#[test]
fn with_error_a() {
    assert_eq!(Err(ErrorWrapper::WrapA(ErrorTypeA::Fail)), try_both(false, false));
}

#[test]
fn with_error_b() {
    assert_eq!(Err(ErrorWrapper::WrapB(ErrorTypeB::Fail)), try_both(true, false));
}

#[test]
fn without_error() {
    assert_eq!(Ok(()), try_both(true, true));
}
