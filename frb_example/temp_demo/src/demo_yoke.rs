use std::borrow::Cow;
use std::rc::Rc;
use yoke::Yoke;

fn load_object(filename: &str) -> Yoke<Cow<'static, str>, Rc<[u8]>> {
    let rc: Rc<[u8]> = load_from_cache(filename);
    Yoke::<Cow<'static, str>, Rc<[u8]>>::attach_to_cart(rc, |data: &[u8]| {
        Cow::Borrowed(&data[1..5])
    })
}

pub fn main() {
    let yoke = load_object("filename.bincode");
    assert_eq!(&**yoke.get(), "hello");
    assert!(matches!(yoke.get(), &Cow::Borrowed(_)));
}
