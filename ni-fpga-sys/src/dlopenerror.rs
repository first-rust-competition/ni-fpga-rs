use std::fmt;

#[derive(Debug)]
pub struct DlOpenError(pub(crate) dlopen::Error);

impl fmt::Display for DlOpenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.0 {
            dlopen::Error::NullCharacter(err) => f.write_fmt(format_args!(
                "Null character in library or function name found {}",
                err
            )),
            dlopen::Error::OpeningLibraryError(err) => {
                f.write_fmt(format_args!("Error opening library {}", err))
            }
            dlopen::Error::SymbolGettingError(err) => {
                f.write_fmt(format_args!("Error loading symbol {}", err))
            }
            dlopen::Error::NullSymbol => f.write_str("Null symbol was found"),
            dlopen::Error::AddrNotMatchingDll(err) => {
                f.write_fmt(format_args!("Address does not match dll {}", err))
            }
        }
    }
}

impl PartialEq for DlOpenError {
    fn eq(&self, other: &Self) -> bool {
        match &self.0 {
            dlopen::Error::NullCharacter(_) => matches!(other.0, dlopen::Error::NullCharacter(_)),
            dlopen::Error::OpeningLibraryError(_) => {
                matches!(other.0, dlopen::Error::OpeningLibraryError(_))
            }
            dlopen::Error::SymbolGettingError(_) => {
                matches!(other.0, dlopen::Error::SymbolGettingError(_))
            }
            dlopen::Error::NullSymbol => matches!(other.0, dlopen::Error::NullSymbol),
            dlopen::Error::AddrNotMatchingDll(_) => {
                matches!(other.0, dlopen::Error::AddrNotMatchingDll(_))
            }
        }
    }
}
