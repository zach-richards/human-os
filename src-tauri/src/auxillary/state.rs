// state.rs

use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;

use crate::sys::system;
use crate::logic::cognitive_model;

pub static SYSTEM_INFO: Lazy<Arc<Mutex<system::SystemInfo>>> =
    Lazy::new(|| Arc::new(Mutex::new(system::SystemInfo::new())));

pub static COGNITIVE_MODEL: Lazy<Arc<Mutex<cognitive_model::CognitiveModel>>> =
    Lazy::new(|| Arc::new(Mutex::new(cognitive_model::CognitiveModel::new())));