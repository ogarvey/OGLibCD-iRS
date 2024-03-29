//! Provides abstractions for the exploration and extraction of CD-i rtf data.
//!
//! This crate provides a number of core abstractions for handling CD-i rtf data. 
//! The primary abstractions are the `CdiFile` and `CdiSector` types. 
//! 
//! The `CdiFile` type represents a CD-i file and provides methods for accessing 
//! the file's data and sectors. 
//! 
//! The `CdiSector` type represents a single sector of a CD-i file and provides methods
//!  for accessing the sector's data, coding and subheader information.

pub mod data;
pub mod helpers;

