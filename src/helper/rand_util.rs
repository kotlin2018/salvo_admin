use rand::Rng;

/// 生成指定长度的字符串
pub fn rand_string(length: usize) ->String{
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    let mut rng = rand::thread_rng();
    let rand_string = (0..length)
        .map(|_|{
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        }).collect();
    rand_string
}