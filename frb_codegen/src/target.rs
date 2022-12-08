use std::iter::FromIterator;

#[derive(Debug, Clone, Copy)]
pub enum Target {
    Common,
    Io,
    Wasm,
}

impl Target {
    #[inline]
    pub const fn is_wasm(&self) -> bool {
        matches!(self, Self::Wasm)
    }
    #[inline]
    pub const fn call_convention(&self) -> &str {
        match self {
            Self::Io => "extern \"C\"",
            _ => "",
        }
    }
    #[inline]
    pub const fn extern_func_attr(&self) -> &str {
        match self {
            Self::Io => "#[no_mangle]",
            Self::Wasm => "#[wasm_bindgen]",
            _ => "",
        }
    }
}

/// Generic accumulator over the targets.
///
/// [`Acc<Option<String>>`] implements <code>[From]\<impl [ToString]></code>
/// for code shared between all platforms.
#[derive(Debug, Default, Clone)]
pub struct Acc<T> {
    pub common: T,
    pub io: T,
    pub wasm: T,
}

impl<T> std::ops::AddAssign for Acc<Vec<T>> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.common.extend(rhs.common);
        self.io.extend(rhs.io);
        self.wasm.extend(rhs.wasm);
    }
}

impl<T> Extend<Acc<T>> for Acc<Vec<T>> {
    fn extend<A: IntoIterator<Item = Acc<T>>>(&mut self, iter: A) {
        for i in iter {
            self.push_acc(i)
        }
    }
}

impl<T> FromIterator<Acc<T>> for Acc<Vec<T>> {
    fn from_iter<A: IntoIterator<Item = Acc<T>>>(iter: A) -> Self {
        iter.into_iter()
            .fold(Acc::<Vec<T>>::default(), |mut acc, x| {
                acc.push_acc(x);
                acc
            })
    }
}

impl<T> Acc<T> {
    pub fn new(mut init: impl FnMut(Target) -> T) -> Acc<T> {
        Acc {
            common: init(Target::Common),
            io: init(Target::Io),
            wasm: init(Target::Wasm),
        }
    }
    pub fn map<O>(self, mut mapper: impl FnMut(T, Target) -> O) -> Acc<O> {
        Acc {
            common: mapper(self.common, Target::Common),
            io: mapper(self.io, Target::Io),
            wasm: mapper(self.wasm, Target::Wasm),
        }
    }
    /// Assign this value to all non-common targets.
    pub fn distribute(value: T) -> Self
    where
        T: Clone + Default,
    {
        Self {
            common: T::default(),
            io: value.clone(),
            wasm: value,
        }
    }
}

impl<T: ToString> From<T> for Acc<Option<String>> {
    #[inline]
    fn from(common: T) -> Self {
        Acc {
            common: Some(common.to_string()),
            ..Default::default()
        }
    }
}

impl<T> Acc<Vec<T>> {
    /// Push to the common buffer.
    #[inline]
    pub fn push(&mut self, common: T) {
        self.common.push(common)
    }

    /// Extend to the common buffer.
    #[inline]
    pub fn extend(&mut self, common: impl IntoIterator<Item = T>) {
        self.common.extend(common)
    }

    #[inline]
    pub fn push_acc(&mut self, acc: Acc<T>) {
        let Acc { common, io, wasm } = acc;
        self.common.push(common);
        self.io.push(io);
        self.wasm.push(wasm);
    }

    #[inline]
    pub fn push_all(&mut self, item: T)
    where
        T: Clone,
    {
        self.common.push(item.clone());
        self.io.push(item.clone());
        self.wasm.push(item);
    }
}

impl Acc<Vec<String>> {
    #[inline]
    pub fn join(&self, sep: &str) -> Acc<String> {
        Acc {
            common: self.common.join(sep),
            io: self.io.join(sep),
            wasm: self.wasm.join(sep),
        }
    }
}
