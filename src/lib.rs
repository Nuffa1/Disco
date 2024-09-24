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
    drivetrain: Subsystem<drivetrain>,
    shooter: Subsystem<shooter>
}

pub struct Controllers {
    left_drive: Joystick,
    right_drive: Joystick,
    operator: Joystick
}

pub async fn configure(executor: &LocalSet) {
    let ferris = Ferris 
    {
        drivetrain: Subsystem::new(Drivetrain::new()),
        shooter: Subsystem::new(Shooter::new()),
    };

    let mut controllers = Controllers 
    {
        left_drive: Joystick::new(constants::input::LEFT),
        right_drive: Joystick::new(constants::input::RIGHT),
        operator: Joystick::new(constants::input::OPERATOR),
    };

    container!(container, executor, &ferris, &mut controllers);
}

pub async fn container<'a>(executor: &'a LocalSet, ferris: &Ferris, controllers: &mut Controllers) {
    
}