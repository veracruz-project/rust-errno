use Errno;

pub const STRERROR_NAME: &'static str = "";

pub fn with_description<F, T>(_err: Errno, callback: F) -> T where
    F: FnOnce(Result<&str, Errno>) -> T
{
    callback(Ok(""))
}

pub fn errno() -> Errno {
    Errno(0)
}

pub fn set_errno(_errno: Errno) {
}
