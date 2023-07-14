fn main() {
    #[cfg(all(feature = "objc", feature = "objc2"))]
    compile_error!("The features `objc` and `objc2` are mutually exclusive. Select one.");

    #[cfg(all(not(feature = "objc"), not(feature = "objc2")))]
    compile_error!("The features `objc` and `objc2` are mutually exclusive. Select one.");

    #[cfg(not(feature = "servo-cf"))]
    compile_error!("The feature `servo-cf` is required.");

    #[cfg(not(any(target_endian = "big", target_endian = "little")))]
    compile_error!("Unknown endianness. Can't target this system.");
}
