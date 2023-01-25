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
    type Error;

    fn set(&mut self, key: u64, val: User);
    fn get(&self, key: &u64) -> Option<&User>;
    fn remove(&mut self, key: &u64) -> Option<User>;
    fn update(&mut self, key: u64, val: User) -> Result<(), Self::Error>;
}

pub struct UserRepoImpl<S: Storage<u64, User>> {
    storage: S,
}

impl<S: Storage<u64, User>> UserRepository for UserRepoImpl<S> {
    type Error = &'static str;

    fn set(&mut self, key: u64, val: User) {
        self.storage.set(key, val)
    }

    fn get(&self, key: &u64) -> Option<&User> {
        self.storage.get(key)
    }

    fn remove(&mut self, key: &u64) -> Option<User> {
        self.storage.remove(key)
    }

    fn update(&mut self, key: u64, val: User) -> Result<(), Self::Error> {
        match self.storage.get(&key) {
            Some(_) => {
                self.storage.set(key, val);
                Ok(())
            }
            None => Err("user not found"),
        }
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
        type Error = &'static str;

        fn set(&mut self, key: u64, val: User) {
            self.storage.set(key, val)
        }

        fn get(&self, key: &u64) -> Option<&User> {
            self.storage.get(key)
        }

        fn remove(&mut self, key: &u64) -> Option<User> {
            self.storage.remove(key)
        }

        fn update(&mut self, key: u64, val: User) -> Result<(), Self::Error> {
            match self.storage.get(&key) {
                Some(_) => {
                    self.storage.set(key, val);
                    Ok(())
                }
                None => Err("user not found"),
            }
        }
    }
}

mod tests {
    use crate::dyn_impl::DynUserRepoImpl;

    use super::*;

    #[test]
    fn non_dynamic_impl_test() {
        let user0 = User {
            id: 0,
            email: Cow::from("test@gmail.com"),
            activated: true,
        };

        let user1 = User {
            id: 1,
            email: Cow::from("test1@gmail.com"),
            activated: false,
        };

        let mut storage: HashMap<u64, User> = Default::default();

        let mut repo = UserRepoImpl { storage };

        repo.set(user0.id, user0.clone());
        assert_eq!(repo.get(&user0.id), Some(&user0.clone()));

        repo.update(0, user1.clone());
        assert_eq!(repo.get(&0), Some(&user1.clone()));
    }

    #[test]
    fn dynamic_impl_test() {
        let user0 = User {
            id: 0,
            email: Cow::from("test@gmail.com"),
            activated: true,
        };

        let user1 = User {
            id: 1,
            email: Cow::from("test1@gmail.com"),
            activated: false,
        };

        let mut strg: HashMap<u64, User> = Default::default();

        let mut repo = DynUserRepoImpl {
            storage: Box::new(strg),
        };

        repo.set(user0.id, user0.clone());
        assert_eq!(repo.get(&user0.id), Some(&user0));

        repo.update(0, user1.clone());
        assert_eq!(repo.get(&0), Some(&user1));
    }
}
