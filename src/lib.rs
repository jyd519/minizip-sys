#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub use bindings::*;
mod bindings;

pub const MZ_OK: i32 = 0; /* zlib */
pub const MZ_STREAM_ERROR: i32 = -1; /* zlib */
pub const MZ_DATA_ERROR: i32 = -3; /* zlib */
pub const MZ_MEM_ERROR: i32 = -4; /* zlib */
pub const MZ_BUF_ERROR: i32 = -5; /* zlib */
pub const MZ_VERSION_ERROR: i32 = -6; /* zlib */

pub const MZ_END_OF_LIST: i32 = -100;
pub const MZ_END_OF_STREAM: i32 = -101;

pub const MZ_PARAM_ERROR: i32 = -102;
pub const MZ_FORMAT_ERROR: i32 = -103;
pub const MZ_INTERNAL_ERROR: i32 = -104;
pub const MZ_CRC_ERROR: i32 = -105;
pub const MZ_CRYPT_ERROR: i32 = -106;
pub const MZ_EXIST_ERROR: i32 = -107;
pub const MZ_PASSWORD_ERROR: i32 = -108;
pub const MZ_SUPPORT_ERROR: i32 = -109;
pub const MZ_HASH_ERROR: i32 = -110;
pub const MZ_OPEN_ERROR: i32 = -111;
pub const MZ_CLOSE_ERROR: i32 = -112;
pub const MZ_SEEK_ERROR: i32 = -113;
pub const MZ_TELL_ERROR: i32 = -114;
pub const MZ_READ_ERROR: i32 = -115;
pub const MZ_WRITE_ERROR: i32 = -116;
pub const MZ_SIGN_ERROR: i32 = -117;
pub const MZ_SYMLINK_ERROR: i32 = -118;

/* MZ_OPEN */
pub const MZ_OPEN_MODE_READ: i32 = 0x01;
pub const MZ_OPEN_MODE_WRITE: i32 = 0x02;
pub const MZ_OPEN_MODE_READWRITE: i32 = MZ_OPEN_MODE_READ | MZ_OPEN_MODE_WRITE;
pub const MZ_OPEN_MODE_APPEND: i32 = 0x04;
pub const MZ_OPEN_MODE_CREATE: i32 = 0x08;
pub const MZ_OPEN_MODE_EXISTING: i32 = 0x10;

/* MZ_SEEK */
pub const MZ_SEEK_SET: i32 = 0;
pub const MZ_SEEK_CUR: i32 = 1;
pub const MZ_SEEK_END: i32 = 2;

/* MZ_COMPRESS */
pub const MZ_COMPRESS_METHOD_STORE: i32 = 0;
pub const MZ_COMPRESS_METHOD_DEFLATE: i32 = 8;
pub const MZ_COMPRESS_METHOD_BZIP2: i32 = 12;
pub const MZ_COMPRESS_METHOD_LZMA: i32 = 14;
pub const MZ_COMPRESS_METHOD_AES: i32 = 99;

pub const MZ_COMPRESS_LEVEL_DEFAULT: i32 = -1;
pub const MZ_COMPRESS_LEVEL_FAST: i32 = 2;
pub const MZ_COMPRESS_LEVEL_NORMAL: i32 = 6;
pub const MZ_COMPRESS_LEVEL_BEST: i32 = 9;

/* MZ_ZIP_FLAG */
pub const MZ_ZIP_FLAG_ENCRYPTED: i32 = 1 << 0;
pub const MZ_ZIP_FLAG_LZMA_EOS_MARKER: i32 = 1 << 1;
pub const MZ_ZIP_FLAG_DEFLATE_MAX: i32 = 1 << 1;
pub const MZ_ZIP_FLAG_DEFLATE_NORMAL: i32 = 0;
pub const MZ_ZIP_FLAG_DEFLATE_FAST: i32 = 1 << 2;
pub const MZ_ZIP_FLAG_DEFLATE_SUPER_FAST: i32 = MZ_ZIP_FLAG_DEFLATE_FAST | MZ_ZIP_FLAG_DEFLATE_MAX;
pub const MZ_ZIP_FLAG_DATA_DESCRIPTOR: i32 = 1 << 3;
pub const MZ_ZIP_FLAG_UTF8: i32 = 1 << 11;
pub const MZ_ZIP_FLAG_MASK_LOCAL_INFO: i32 = 1 << 13;

/* MZ_ZIP_EXTENSION */
pub const MZ_ZIP_EXTENSION_ZIP64: i32 = 0x0001;
pub const MZ_ZIP_EXTENSION_NTFS: i32 = 0x000a;
pub const MZ_ZIP_EXTENSION_AES: i32 = 0x9901;
pub const MZ_ZIP_EXTENSION_UNIX1: i32 = 0x000d;
pub const MZ_ZIP_EXTENSION_SIGN: i32 = 0x10c5;
pub const MZ_ZIP_EXTENSION_HASH: i32 = 0x1a51;
pub const MZ_ZIP_EXTENSION_CDCD: i32 = 0xcdcd;

/* MZ_ZIP64 */
pub const MZ_ZIP64_AUTO: i32 = 0;
pub const MZ_ZIP64_FORCE: i32 = 1;
pub const MZ_ZIP64_DISABLE: i32 = 2;

/* MZ_HOST_SYSTEM */
// pub const MZ_HOST_SYSTEM(VERSION_MADEBY)  ((uint8_t)(VERSION_MADEBY >> 8))
pub const MZ_HOST_SYSTEM_MSDOS: i32 = 0;
pub const MZ_HOST_SYSTEM_UNIX: i32 = 3;
pub const MZ_HOST_SYSTEM_WINDOWS_NTFS: i32 = 10;
pub const MZ_HOST_SYSTEM_RISCOS: i32 = 13;
pub const MZ_HOST_SYSTEM_OSX_DARWIN: i32 = 19;

/* MZ_PKCRYPT */
pub const MZ_PKCRYPT_HEADER_SIZE: i32 = 12;

/* MZ_AES */
pub const MZ_AES_VERSION: i32 = 1;
pub const MZ_AES_ENCRYPTION_MODE_128: i32 = 0x01;
pub const MZ_AES_ENCRYPTION_MODE_192: i32 = 0x02;
pub const MZ_AES_ENCRYPTION_MODE_256: i32 = 0x03;
// pub const MZ_AES_KEY_LENGTH(MODE)         (8 * (MODE & 3) + 8)
pub const MZ_AES_KEY_LENGTH_MAX: i32 = 32;
pub const MZ_AES_BLOCK_SIZE: i32 = 16;
// pub const MZ_AES_HEADER_SIZE(MODE)        ((4 * (MODE & 3) + 4) + 2)
pub const MZ_AES_FOOTER_SIZE: i32 = 10;

/* MZ_HASH */
pub const MZ_HASH_MD5: i32 = 10;
pub const MZ_HASH_MD5_SIZE: i32 = 16;
pub const MZ_HASH_SHA1: i32 = 20;
pub const MZ_HASH_SHA1_SIZE: i32 = 20;
pub const MZ_HASH_SHA256: i32 = 23;
pub const MZ_HASH_SHA256_SIZE: i32 = 32;
pub const MZ_HASH_MAX_SIZE: i32 = 256;

/* MZ_ENCODING */
pub const MZ_ENCODING_CODEPAGE_437: i32 = 437;
pub const MZ_ENCODING_CODEPAGE_932: i32 = 932;
pub const MZ_ENCODING_CODEPAGE_936: i32 = 936;
pub const MZ_ENCODING_CODEPAGE_950: i32 = 950;
pub const MZ_ENCODING_UTF8: i32 = 65001;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
