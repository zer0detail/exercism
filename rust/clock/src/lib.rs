use std::{fmt::Display, ops::{Add, Sub}};

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", &self.hours, &self.minutes)
    }
}

impl Clock {

    fn wind_hours_up(hours: &mut i32) {
        while *hours < 0 {
            *hours += 24;
        }
    }
    fn wind_hours_down(hours: &mut i32) {
        while *hours > 24 {
            *hours -= 24;
        }
        if *hours == 24 { *hours = 0; }
    }
    fn wind_minutes_up(hours: &mut i32, minutes: &mut i32){
        while *minutes < 0 {
            *minutes += 60;
            *hours -= 1;
            if *hours < 0 { *hours = 23;}
            
        }
        if *minutes == 60 { *minutes = 0; *hours += 1; }
        if *hours == 24 { *hours = 0; }
    }
    fn wind_minutes_down(hours: &mut i32, minutes: &mut i32) {
        while *minutes > 60 { 
            *minutes  -= 60; 
            *hours += 1;
            if *hours >= 24 {
                *hours = 0;
            }
        }
        if *minutes == 60 { *minutes = 0; *hours += 1; }
        if *hours == 24 { *hours = 0; }
    }
    pub fn new(mut hours: i32, mut minutes: i32) -> Self {
        match hours {
            h if h > 24 => Clock::wind_hours_down(&mut hours),
            h if h < 24 => Clock::wind_hours_up(&mut hours),
            h if h == 24 => hours = 0,
            _ => (),
        };
        match minutes {
            m if m < 0 => Clock::wind_minutes_up(&mut hours, &mut minutes),
            m if m >= 60 => Clock::wind_minutes_down(&mut hours, &mut minutes),
            _ => (),
        };
        Self { hours, minutes }
    }

    pub fn add_minutes(&mut self, minutes: i32) -> Self {
        self.minutes += minutes;
        
        match self.minutes {
            m if m < 0 => Clock::wind_minutes_up(&mut self.hours, &mut self.minutes),
            m if m >= 60 => Clock::wind_minutes_down(&mut self.hours, &mut self.minutes),
            _ => (),
        };
        Self {
            hours: self.hours,
            minutes: self.minutes,
        }
    }
}
