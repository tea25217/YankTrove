/// Decodes yt-dlp stdout/stderr chunks that may split UTF-8 or use CP932 on Windows.
pub struct StreamDecoder {
    pending: Vec<u8>,
}

impl StreamDecoder {
    pub fn new() -> Self {
        Self {
            pending: Vec::new(),
        }
    }

    pub fn push(&mut self, chunk: &[u8]) -> String {
        if chunk.is_empty() {
            return String::new();
        }
        self.pending.extend_from_slice(chunk);
        let complete_len = complete_prefix_len(&self.pending);
        if complete_len == 0 {
            return String::new();
        }
        let complete: Vec<u8> = self.pending.drain(..complete_len).collect();
        decode_bytes(&complete)
    }

    pub fn finish(&mut self) -> String {
        if self.pending.is_empty() {
            return String::new();
        }
        let leftover = std::mem::take(&mut self.pending);
        decode_bytes(&leftover)
    }
}

fn complete_prefix_len(buf: &[u8]) -> usize {
    if buf.is_empty() {
        return 0;
    }

    let mut i = buf.len();
    let mut continuation = 0;
    while i > 0 && continuation < 3 && buf[i - 1] & 0xC0 == 0x80 {
        i -= 1;
        continuation += 1;
    }
    if i == 0 {
        return 0;
    }

    let lead = buf[i - 1];
    if lead < 0x80 {
        return buf.len();
    }

    let needed = match lead {
        0xC0..=0xDF => 2,
        0xE0..=0xEF => 3,
        0xF0..=0xF7 => 4,
        _ => return buf.len(),
    };
    let have = buf.len() - (i - 1);
    if have < needed {
        i - 1
    } else {
        buf.len()
    }
}

fn decode_bytes(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return String::new();
    }
    if let Ok(text) = std::str::from_utf8(bytes) {
        return text.to_string();
    }

    encoding_rs::SHIFT_JIS.decode(bytes).0.into_owned()
}
