/*
** Configuration file with all constants used in Randetect
*/

/* Maximum log parsed starting from last entry */
pub const MAX_LOG: u16 = 2_500;

/* Loop dealy in milliseconds */
pub const TIME: u64 = 2_000;

/* Path to Synology logs database */
//pub const DBPATH: &str = "/var/log/synolog/";
pub const DBPATH: &str = "/home/antoine/RanDetect/"; // For dev
pub const DBNAME: &str = ".SMBXFERDB";

/* Randetect error log files */
pub const LOGDIR: &str = "/var/log/randetect/";
pub const ERRLOG: &str = "error.log";
pub const OUTLOG: &str = "output.log";

/* Maximum suspicious action limit */
pub const BAN_LIMIT: u16 = 50;
