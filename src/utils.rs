// copy from: https://github.com/helix-editor/helix/blob/5f04d09f030da0711b92894bd8dd6623135882ac/helix-core/src/fuzzy.rs
use std::ops::DerefMut;

use nucleo::{pattern::{Atom, AtomKind, CaseMatching, Normalization}, Config};
use parking_lot::Mutex;

struct LazyMutex<T> {
    inner: Mutex<Option<T>>,
    init: fn() -> T,
}

impl<T> LazyMutex<T> {
    const fn new(init: fn() -> T) -> Self {
        Self {
            inner: Mutex::new(None),
            init,
        }
    }

    fn lock(&self) -> impl DerefMut<Target = T> + '_ {
        parking_lot::MutexGuard::map(self.inner.lock(), |val| val.get_or_insert_with(self.init))
    }
}

static MATCHER: LazyMutex<nucleo::Matcher> = LazyMutex::new(nucleo::Matcher::default);

pub fn fuzzy_match<T: AsRef<str>>(
    pattern: &str,
    items: impl IntoIterator<Item = T>,
) -> Vec<(T, u16)> {
    let mut matcher = MATCHER.lock();
    matcher.config = Config::DEFAULT;
    let pattern = Atom::new(pattern, CaseMatching::Smart, Normalization::Smart, AtomKind::Fuzzy, false);
    pattern.match_list(items, &mut matcher)
}
