use lazy_static::lazy_static;
use std::ffi::CString;
use std::{collections::HashMap, sync::RwLock};
use uuid::Uuid;

pub fn generate_class_name(debug_name: &str) -> String {
    #[cfg(debug_assertions)]
    return format!("{}_{}_{}", debug_name, Uuid::new_v4(), "apple_rs_debug");
    #[cfg(not(debug_assertions))]
    return format!("{}_{}", Uuid::new_v4(), "apple_rs");
}

#[macro_export]
macro_rules! class_name {
    ($debug_name: expr) => {
        $crate::utils::generate_class_name($debug_name)
    };
}

lazy_static! {
    /// A cache of classes
    pub static ref CLASS_CACHE: ClassCache = ClassCache::new();
}

use super::objective::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// A key for a class
pub struct ClassKey {
    /// The name of the class
    pub name: String,
    /// The name of the superclass
    pub superclass: Option<String>,
}

impl ClassKey {
    /// Create a new class key
    pub fn new(name: String, superclass: Option<String>) -> Self {
        Self { name, superclass }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// A key for a class
pub struct ClassData {
    /// The class pointer
    pub class: usize,
}

impl ClassData {
    /// Get the class pointer
    pub fn ptr(&self) -> *const runtime::Class {
        self.class as *const runtime::Class
    }
}

#[derive(Debug)]
/// A cache of classes
pub struct ClassCache {
    cache: RwLock<HashMap<ClassKey, ClassData>>,
}

impl ClassCache {
    /// Create a new class cache
    pub(super) fn new() -> Self {
        Self {
            cache: RwLock::new(HashMap::new()),
        }
    }

    /// Get a class from the cache
    pub fn load(&self, class_key: ClassKey) -> Option<*const runtime::Class> {
        let cache = match self.cache.read() {
            Ok(cache) => cache,
            Err(_) => return None,
        };

        if let Some(cached_class) = cache.get(&class_key) {
            return Some(cached_class.ptr());
        }

        // Load key :P
        let c_class = match CString::new(class_key.name.clone()) {
            Ok(c_class) => c_class,
            Err(_) => return None,
        };
        let class = unsafe { runtime::objc_getClass(c_class.as_ptr()) };

        if class.is_null() {
            return None;
        }

        self.save(
            class_key,
            ClassData {
                class: class as usize,
            },
        );

        Some(class)
    }

    pub fn save(&self, class_key: ClassKey, class_data: ClassData) -> Option<()> {
        let mut cache = match self.cache.write() {
            Ok(cache) => cache,
            Err(_) => return None,
        };

        cache.insert(class_key, class_data);

        Some(())
    }
}

#[inline]
/// Get or load a class
pub fn get_class<T>(
    super_class: &str,
    class_name: &str,
    generator: T,
) -> Option<*const runtime::Class>
where
    T: Fn(&mut declare::ClassDecl),
{
    if let Some(class) = CLASS_CACHE.load(ClassKey::new(
        class_name.to_string(),
        Some(super_class.to_string()),
    )) {
        return Some(class);
    }

    // Load the superclass
    let super_class_id = match CLASS_CACHE.load(ClassKey::new(super_class.to_string(), None)) {
        Some(super_class) => super_class,
        None => {
            #[cfg(debug_assertions)]
            panic!(
                "Failed to register class '{}' because of the superclass '{}' failed to load",
                class_name, super_class
            );
            #[cfg(not(debug_assertions))]
            return None;
        }
    };

    // Unique class name
    let unique_name = format!("{}_{}_{}", super_class, class_name, Uuid::new_v4());

    // Create the class
    let decl = declare::ClassDecl::new(&unique_name, unsafe { &*super_class_id });

    match decl {
        Some(mut decl) => {
            generator(&mut decl);

            let class = decl.register();

            match CLASS_CACHE.save(
                ClassKey::new(class_name.to_string(), Some(super_class.to_string())),
                ClassData {
                    class: class as *const runtime::Class as usize,
                },
            ) {
                Some(_) => Some(class),
                None => None,
            }
        }
        None => {
            #[cfg(debug_assertions)]
            panic!(
                "Failed to declate class '{}' because the class declaration failed",
                class_name
            );
            #[cfg(not(debug_assertions))]
            None
        }
    }
}
