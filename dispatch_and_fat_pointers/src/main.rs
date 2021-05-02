fn main() {
    println!("Hello, world!");
    println!("{:?}", str_len("Hello, world"));
    println!("{:?}", str_len1("Hello, world"));

    "hello world".hi();

    foo_arr();

    baz_arr(&["hello", "world"]);
    baz_arr_generic(&["mybank", "network"]);
}

pub fn str_len(s: impl AsRef<str>) -> usize {
    s.as_ref().len()
}

pub fn str_len1<S>(s: S) -> usize
where
    S: AsRef<str>,
{
    s.as_ref().len()
}

// 注意大量使用模版函数生成过多实例的现象
// when compiling
pub fn str_len_ref_str(s: &str) -> usize {
    s.len()
}

// when compiling
pub fn str_len_string(s: String) -> usize {
    s.len()
}

// trait
pub trait Hi {
    fn hi(&self);
}

impl Hi for &str {
    fn hi(&self) {
        println!("hi {}", self)
    }
}

pub fn bar(h: impl Hi) {
    h.hi();
}

pub fn baz<H: Hi>(h: H) {
    h.hi();
}

pub fn foo_arr() {
    for h in vec!["aaron", "anderson"] {
        h.hi();
    }
}

pub fn baz_arr(s: &[impl Hi]) {
    for h in s {
        h.hi();
    }
}

pub fn baz_arr_generic<H: Hi>(s: &[H]) {
    for h in s {
        h.hi();
    }
}
