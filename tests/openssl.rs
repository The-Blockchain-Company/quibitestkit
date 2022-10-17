use assert_fs::TempDir;

#[test]
pub fn openssl_e2e() {
    let temp_dir = TempDir::new().unwrap();
    let (_, _, _) = quibitestkit::openssl::generate_keys(&temp_dir);
}
