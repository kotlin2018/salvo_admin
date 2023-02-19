use crypto::digest::Digest;
use crypto::md5::Md5;

pub fn make_md5(data: &str) -> String {
    let mut hasher = Md5::new();
    hasher.input_str(data);
    hasher.result_str()
}
