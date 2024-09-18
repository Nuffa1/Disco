pub mod subsystems;
pub mod constants;

use std::cell::RefCell;
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::rc::Rc;
use frcrs::container;
use frcrs::input::Joystick;
use tokio::task::LocalSet;
use crate::subsystems::{Drivetrain, Shooter};
use frcrs::sleep_hz;
use frcrs::Subsystem;

pub struct Ferris {
    
}

pub struct Controllers {
    
}

pub async fn configure(executor: &LocalSet) {
    

    container!(container, executor, &ferris, &mut controllers);
}

pub async fn container<'a>(executor: &'a LocalSet, ferris: &Ferris, controllers: &mut Controllers) {
    
}