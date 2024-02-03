use crate::tree::TreeDisplay;
use std::fmt::Display;
use std::ops::Deref;
use std::path::Path;

pub trait StartsWith {
    type Output;
    fn starts_with(&self, other: &Self) -> bool;
    fn strip_prefix(&self, other: &Self) -> Option<Self::Output>;
}

impl<'a> StartsWith for &'a str {
    type Output = &'a str;

    fn starts_with(&self, other: &Self) -> bool {
        str::starts_with(self, other)
    }

    fn strip_prefix(&self, other: &Self) -> Option<Self::Output> {
        str::strip_prefix(self, other)
    }
}

impl<'a> StartsWith for &'a Path {
    type Output = &'a Path;

    fn starts_with(&self, other: &Self) -> bool {
        Path::starts_with(self, other)
    }

    fn strip_prefix(&self, other: &Self) -> Option<Self::Output> {
        Path::strip_prefix(self, other).ok()
    }
}

pub fn display_iter<T: ?Sized, Q: Deref<Target = T>>(
    iter: impl IntoIterator<Item = Q>,
) -> impl Iterator<Item = String>
where
    for<'a> &'a T: StartsWith + Display,
    for<'a> <&'a T as StartsWith>::Output: Display,
{
    let iter = iter.into_iter().peekable();
    let mut prefixes = Vec::<Q>::new();
    let mut tree = TreeDisplay::default();

    for item in iter {
        let item: Q = item;
        while let Some(prefix) = prefixes.last() {
            let Some(rest) = item.deref().strip_prefix(&prefix.deref()) else {
                prefixes.pop();
                tree.pop_layer().expect("Should have enough layers");
                continue;
            };
            tree.add_child(rest.to_string());
            break;
        }
        if prefixes.is_empty() {
            tree.add_child(item.deref().to_string());
        }
        prefixes.push(item);
        tree.push_layer().expect("Should have parent layer");
    }
    tree.lines().into_iter()
}
