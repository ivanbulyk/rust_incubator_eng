#![allow(unused)]

use std::hash::Hash;
use std::{borrow::Cow, collections::HashMap};

pub trait Storage<K, V> {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}

impl<K: Hash + Eq, V> Storage<K, V> for HashMap<K, V> {
    fn set(&mut self, key: K, val: V) {
        self.insert(key, val);
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.get(key)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.remove(key)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct User {
    pub id: u64,
    pub email: Cow<'static, str>,
    pub activated: bool,
}

pub trait UserRepository {
    fn set(&mut self, key: u64, val: User);
    fn get(&self, key: &u64) -> Option<&User>;
    fn remove(&mut self, key: &u64) -> Option<User>;
}

pub struct UserRepoImpl<S: Storage<u64, User>> {
    storage: S,
}

impl<S: Storage<u64, User>> UserRepository for UserRepoImpl<S> {
    fn set(&mut self, key: u64, val: User) {
        self.storage.set(key, val)
    }

    fn get(&self, key: &u64) -> Option<&User> {
        self.storage.get(key)
    }

    fn remove(&mut self, key: &u64) -> Option<User> {
        self.storage.remove(key)
    }
}

mod dyn_impl {
    use crate::{Storage, User, UserRepository};

    impl<K: Eq, V> Storage<K, V> for Vec<(K, V)> {
        fn set(&mut self, key: K, val: V) {
            self.push((key, val))
        }

        fn get(&self, key: &K) -> Option<&V> {
            for (k, v) in self.iter() {
                if k == key {
                    return Some(v);
                }
            }
            None
        }

        fn remove(&mut self, key: &K) -> Option<V> {
            for (i, (k, v)) in self.iter_mut().enumerate() {
                if k == key {
                    return Some(self.remove(i).1);
                }
            }
            None
        }
    }

    pub struct DynUserRepoImpl {
        pub(crate) storage: Box<dyn Storage<u64, User>>,
    }

    impl UserRepository for DynUserRepoImpl {
        fn set(&mut self, key: u64, val: User) {
            self.storage.set(key, val)
        }

        fn get(&self, key: &u64) -> Option<&User> {
            self.storage.get(key)
        }

        fn remove(&mut self, key: &u64) -> Option<User> {
            self.storage.remove(key)
        }
    }
}

mod tests {
    use crate::dyn_impl::DynUserRepoImpl;

    use super::*;

    #[test]
    fn non_dynamic_impl_test() {
        let user1 = User {
            id: 0,
            email: Cow::from("test@gmail.com"),
            activated: true,
        };

        let mut storage: HashMap<u64, User> = Default::default();

        let mut repo = UserRepoImpl { storage };

        repo.set(user1.id, user1.clone());

        assert_eq!(repo.get(&user1.id), Some(&user1));
    }
}
