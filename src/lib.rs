//! A pointer backed by disk.

struct Disk<T>
{
    data: Option<T>,
    file: PathBuf,
}

impl<T> Disk<T>
{
    pub fn new(t: T) -> Self
    {
        // create temp file
        // write t to file
        // return self
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
