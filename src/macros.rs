#[macro_export]
macro_rules! hash_map {
    () => { std::collections::HashMap::with_capacity(16) };

    ($key: expr => $value: expr) => {
        hash_map!($key => $value; 16)
    };
    ($key: expr => $value: expr; $init_capacity: expr) => {
        {
            let mut hash_map = std::collections::HashMap::with_capacity($init_capacity);
            hash_map.insert($key, $value);
            hash_map
        }
    };

    ($($key: expr => $value: expr),*) => {
        vec![$(($key, $value)),*].into_iter().collect::<std::collections::HashMap<_, _>>()
    };
    ($($key: expr => $value: expr,)*) => {
        hash_map!($($key => $value),*)
    };
}

#[macro_export]
macro_rules! btree_map {
    () => { std::collections::BTreeMap::new() };

    ($key: expr => $value: expr) => {
        let mut map = std::collections::BTreeMap::new();
        map.insert($key, $value);
        map
    };

    ($($key: expr => $value: expr),*) => {
        {
            use std::iter::FromIterator;
            BTreeMap::from_iter(vec![$(($key, $value)),*])
        }
    };
    ($($key: expr => $value: expr,)*) => {
        btree_map!($($key => $value),*)
    };
}

#[macro_export]
macro_rules! hash_set {
    () => { std::collections::HashSet::new() };

    ($($elements: expr),*) => {
        {
            let mut set = hash_set!();
            $(set.insert($elements);)*
            set
        }
    };

    ($($elements: expr,)*) => {
        hash_set!($($elements),*)
    };
}

#[macro_export]
macro_rules! btree_set {
    () => { std::collections::BTreeSet::new() };

    ($($elements: expr),*) => {
        {
            let mut set = btree_set!();
            $(set.insert($elements);)*
            set
        }
    };

    ($($elements: expr,)*) => {
        btree_set!($($elements),*)
    };
}
