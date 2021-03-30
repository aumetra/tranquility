use {
    crate::error::{Error, Result},
    http::header::{AsHeaderName, HeaderMap, HeaderValue},
    std::{borrow::Cow, collections::HashMap, hash::Hash, iter::FromIterator},
};

pub trait CowHeaderMapExt {
    /// Convenience function. Is equivalent to `.as_ref().get().ok_or(Error::MissingHeader)`
    fn get_header<K>(&self, key: K) -> Result<&HeaderValue>
    where
        K: AsHeaderName;
}

impl<'a> CowHeaderMapExt for Cow<'a, HeaderMap> {
    fn get_header<K>(&self, key: K) -> Result<&HeaderValue>
    where
        K: AsHeaderName,
    {
        self.as_ref().get(key).ok_or(Error::MissingHeader)
    }
}

pub trait IteratorExt: Iterator + Sized {
    /// Convenience function. Is equivalent to `.collect::<HashMap<_, _>>()`
    fn collect_hashmap<K, V>(self) -> HashMap<K, V>
    where
        HashMap<K, V>: FromIterator<<Self as Iterator>::Item>,
    {
        self.collect()
    }

    /// Convenience function. Is equivalent to `.collect::<Vec<_>>()`
    fn collect_vec(self) -> Vec<<Self as Iterator>::Item> {
        self.collect()
    }

    /// Convenience function. Is equivalent to `.collect::<Result<Vec<_>, _>>()`
    fn try_collect_vec<T, E>(self) -> Result<Vec<T>, E>
    where
        Result<Vec<T>, E>: FromIterator<<Self as Iterator>::Item>,
    {
        self.collect()
    }
}

impl<T> IteratorExt for T where T: Iterator + Sized {}

pub trait HashMapExt<K, V> {
    /// Convenience function. Is equivalent to `.get().copied().ok_or(Error::MissingSigStrField)`
    fn get_signature_field(&self, key: K) -> Result<V>;
}

impl<K, V> HashMapExt<K, V> for HashMap<K, V>
where
    K: Eq + Hash,
    V: Copy,
{
    fn get_signature_field(&self, key: K) -> Result<V> {
        self.get(&key).copied().ok_or(Error::MissingSignatureField)
    }
}
