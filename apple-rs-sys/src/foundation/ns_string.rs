use std::{ffi::c_void, slice};

use crate::utils::*;

#[repr(usize)]
/// # NSStringEncoding
/// A constant that identifies a text encoding.
///
/// `NSUTF16StringEncoding` is missing, but, it's the same as `NSUnicodeStringEncoding`.
pub enum NSStringEncoding {
    /* 7 and 8 bit string encodings */
    /// 7-bit ASCII encoding
    NSASCIIStringEncoding = 1,

    /// 8-bit Latin-1 encoding (ISO 8859-1)
    NSISOLatin1StringEncoding = 5,

    /// 8-bit Latin-2 encoding (ISO 8859-2)
    NSISOLatin2StringEncoding = 9,

    /// 8-bit Classic Macintosh Roman encoding
    NSMacOSRomanStringEncoding = 30,

    /// 8-bit ASCII with NEXTSTEP extensions
    NSNEXTSTEPStringEncoding = 2,

    /// 7-bit lossy verbose ASCII that represents all Unicode characters
    NSNonLossyASCIIStringEncoding = 7,

    /// 8-bit Adobe Symbol Encoding
    NSSymbolStringEncoding = 6,

    /* 7 and 8 bit japanese string encodings */
    /// 8-bit Japanese encoding for mail (ISO 2022-JP)
    NSISO2022JPStringEncoding = 21,

    /// 8-bit EUC encoding for Japanese
    NSJapaneseEUCStringEncoding = 3,

    /// 8-bit Shift-JIS encoding for Japanese
    NSShiftJISStringEncoding = 8,

    /* Unicode string encodings */
    /// 8-bit Unicode Transformation Format (UTF-8)
    NSUTF8StringEncoding = 4,

    /// 16-bit Unicode big-gndian specific encoding (UTF-16)
    NSUTF16BigEndianStringEncoding = 0x90000100,

    /// 16-bit Unicode little-endian specific encoding (UTF-16)
    NSUTF16LittleEndianStringEncoding = 0x94000100,

    /// 16-bit standard Unicode encoding (UTF-16)
    /// Also known as `NSUnicodeStringEncoding`
    NSUnicodeStringEncoding = 10,

    /// 32-bit Unicode big-endian specific encoding (UTF-32)
    NSUTF32BigEndianStringEncoding = 0x98000100,

    /// 32-bit Unicode little-endian specific encoding (UTF-32)
    NSUTF32LittleEndianStringEncoding = 0x9c000100,

    /// 32-bit non-specific endian Unicode encoding (UTF-32)
    NSUTF32StringEncoding = 0x8c000100,

    /* Windows Code Page String Encodings */
    /// Microsoft Windows codepage 1250 encoding (WinLatin2)
    NSWindowsCP1250StringEncoding = 15,

    /// Microsoft Windows codepage 1251 encoding for Cyrillic text (AdobeStandardCyrillic)
    NSWindowsCP1251StringEncoding = 11,

    /// Microsoft Windows codepage 1252 encoding (WinLatin1)
    NSWindowsCP1252StringEncoding = 12,

    /// Microsoft Windows codepage 1253 encoding for Greek text
    NSWindowsCP1253StringEncoding = 13,

    /// Microsoft Windows codepage 1254 encoding for Turkish text
    NSWindowsCP1254StringEncoding = 14,
}

#[derive(Debug)]
/// # NSString
/// A static string object that is represented by a sequence of Unicode characters.
/// `NSString` is equivalent to `CFStringRef` in the Core Foundation framework.
pub struct NSString(pub UnsafeId);

impl NSString {
    /// Init empty string
    pub fn init() -> Self {
        let obj: UnsafeId = unsafe { msg_send![class!(NSString), alloc] };
        let obj: UnsafeId = unsafe { msg_send![obj, init] };

        Self(obj)
    }

    /// Init string from bytes
    pub fn init_with_bytes(
        bytes: *const c_void,
        length: usize,
        encoding: NSStringEncoding,
    ) -> Self {
        let obj: UnsafeId = unsafe { msg_send![class!(NSString), alloc] };
        let obj: UnsafeId =
            unsafe { msg_send![obj, initWithBytes: bytes length: length encoding: encoding] };

        Self(obj)
    }

    /// Init string from bytes without copying
    pub fn init_with_bytes_no_copy(
        bytes: *const c_void,
        length: usize,
        encoding: NSStringEncoding,
    ) -> Self {
        let obj: UnsafeId = unsafe { msg_send![class!(NSString), alloc] };
        let obj: UnsafeId = unsafe {
            msg_send![
                obj,
                initWithBytesNoCopy: bytes
                length: length
                encoding: encoding
                freeWhenDone: Boolean(false).id()
            ]
        };

        Self(obj)
    }

    pub fn length_of_bytes_using_encoding(&self, encoding: NSStringEncoding) -> usize {
        unsafe { msg_send![self.0, lengthOfBytesUsingEncoding: encoding] }
    }

    /// Get the string's C representation with a given encoding
    pub fn utf8_string(&self) -> *const c_void {
        unsafe { msg_send![self.0, UTF8String] }
    }

    // TODO: Rest of the methods

    /// Create a new `NSString` from a value
    pub fn new<T>(value: T) -> Self
    where
        T: Into<NSString>,
    {
        value.into()
    }

    #[inline]
    /// Get a &str slice from the string
    pub fn as_str(&self) -> Option<&str> {
        let bytes = self.to_slice();

        std::str::from_utf8(bytes).ok()
    }

    #[inline]
    /// Get a String from the string
    pub fn to_string(self) -> Option<String> {
        let bytes = self.to_slice();

        String::from_utf8(bytes.to_vec()).ok()
    }

    #[inline]
    pub fn to_slice(&self) -> &[u8] {
        let bytes: *const c_void = self.utf8_string();
        let length: usize =
            self.length_of_bytes_using_encoding(NSStringEncoding::NSUTF8StringEncoding);

        unsafe { slice::from_raw_parts(bytes as *const u8, length) }
    }

    pub fn id(self) -> UnsafeId {
        self.0
    }
}

impl From<&str> for NSString {
    fn from(value: &str) -> Self {
        Self::init_with_bytes(
            value.as_ptr() as *const c_void,
            value.len(),
            NSStringEncoding::NSUTF8StringEncoding,
        )
    }
}

impl From<String> for NSString {
    fn from(value: String) -> Self {
        Self::init_with_bytes_no_copy(
            value.as_ptr() as *const c_void,
            value.len(),
            NSStringEncoding::NSUTF8StringEncoding,
        )
    }
}

impl TryInto<String> for NSString {
    type Error = ();

    fn try_into(self) -> Result<String, Self::Error> {
        self.to_string().ok_or(())
    }
}

impl IsType for NSString {
    type Error = ();

    fn is_type(obj: UnsafeId) -> Result<bool, Self::Error> {
        let raw: UnsafeBool = unsafe { msg_send![obj, isKindOfClass: class!(NSString)] };
        let is: Boolean = raw.try_into()?;

        Ok(is.into())
    }
}

impl TryFrom<UnsafeId> for NSString {
    type Error = ();

    fn try_from(obj: UnsafeId) -> Result<Self, Self::Error> {
        if !Self::is_type(obj)? {
            #[cfg(debug_assertions)]
            panic!("Object is not NSString: {:?}", obj);
            #[cfg(not(debug_assertions))]
            return Err(());
        }

        let string: UnsafeId = unsafe { msg_send![obj, copy] };

        Ok(Self(string))
    }
}

#[cfg(test)]
mod tests {
    use super::NSString;

    #[test]
    fn test_string() {
        let string = NSString::from("Hello, world!");

        assert_eq!(string.as_str(), Some("Hello, world!"));
    }
}