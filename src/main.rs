use std::{
    os::unix::prelude::PermissionsExt,
    path::{self, Path},
};

trait FileMetadata {
    fn exists(&self) -> bool;

    fn is_writeable(&self) -> bool;

    fn is_readable(&self) -> bool;
}

fn file_mode(path: &Path) -> std::io::Result<u32> {
    path.metadata().map(|m| m.permissions().mode())
}

impl FileMetadata for path::Path {
    fn is_readable(&self) -> bool {
        if let Ok(mode) = file_mode(self) {
            (1 & (mode >> 8)) == 1
        } else {
            false
        }
    }

    fn is_writeable(&self) -> bool {
        if let Ok(mode) = file_mode(self) {
            (1 & (mode >> 7)) == 1
        } else {
            false
        }
    }

    fn exists(&self) -> bool {
        self.exists()
    }
}

fn main() {
    use std::fs;

    let f = Path::new("Cargo.toml");
    let mode = f.metadata().unwrap().permissions().mode();
    println!("{:b}", mode);
    println!("{:b}", 1 & (mode >> 2));
    // assert!(f.path().is_writeable());

    // fs::remove_file(f.path()).unwrap();
}

#[test]
fn writeable() {
    use std::fs;
    use tempfile;

    let f = tempfile::NamedTempFile::new().unwrap();
    assert_eq!(f.path().is_writeable(), true);

    fs::remove_file(f.path()).unwrap();
}

#[test]
fn read_only() {
    use std::fs;
    use tempfile;

    let f = tempfile::NamedTempFile::new().unwrap();
    let mut perms = fs::metadata(f.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(f.path(), perms).unwrap();
    assert_eq!(f.path().is_writeable(), false);

    fs::remove_file(f.path()).unwrap();
}
