use crate::Options;

const CAROUSEL_CHARS: &[char] = &['-', '\\', '|', '/'];

pub type TaskResult = Result<(), std::io::Error>;
pub const CLREOL: &[u8] = &[0x1b, b'[', b'0', b'G', 0x1b, b'[', b'0', b'K'];

pub(crate) struct Carousel {
    buf: Vec<u8>,
    reverse: bool,
    buf_idx: usize,
    c_idx: usize,
}

impl Carousel {
    pub(crate) fn new(opts: Options) -> Self {
        let mut buf = vec![0x1b, b'[', b'D'];
        if let Some(c) = opts.color {
            buf.extend(c.as_escape());
        }
        if opts.bold {
            buf.extend([0x1b, b'[', b'1', b'm']);
        }
        let buf_idx = buf.len();
        buf.push(0);
        if opts.color.is_some() || opts.bold {
            buf.extend(vec![0x1b, b'[', b'0', b'm']);
        };
        Self {
            buf,
            reverse: opts.reverse,
            buf_idx,
            c_idx: 0,
        }
    }
    #[allow(dead_code)]
    #[inline]
    pub(crate) fn rotate(&'_ mut self) -> &'_ [u8] {
        self.buf[self.buf_idx] = CAROUSEL_CHARS[if self.reverse {
            CAROUSEL_CHARS.len() - self.c_idx - 1
        } else {
            self.c_idx
        }] as u8;
        self.c_idx += 1;
        if self.c_idx == CAROUSEL_CHARS.len() {
            self.c_idx = 0;
        }
        &self.buf
    }
}

#[inline]
pub(crate) fn cleanup(res: impl std::fmt::Display) {
    let s = res.to_string();
    if atty::is(atty::Stream::Stdout) {
        print!("\x1b[D");
        if s.is_empty() {
            print!(" ");
        }
    }
    println!("{}", s);
}
