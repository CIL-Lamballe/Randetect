/*
** Configuration file with all constants used in Randetect
*/

/* Maximum log parsed - LIFO. */
pub const MAX_LOG: u16 = 2_000;

/* Loop dealy in milliseconds. */
pub const TIME: u64 = 2_000;

/* Path to Synology logs. */
pub const DBPATH: &str = "/var/log/synolog/";

/* Database containing file logs. */
pub const DB: &str = ".SMBXFERDB";
