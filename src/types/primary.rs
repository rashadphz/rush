use chrono::{DateTime, Datelike, Timelike, Utc};
use core::fmt::Debug;
use std::{borrow::Borrow, time::SystemTime};

use crate::views::baseview::BaseView;

use super::descriptor::Descriptor;

pub trait RshObject: Debug {
    fn data_descriptors(&self) -> Vec<Descriptor>;
    fn get_data(&self, desc: &Descriptor) -> &Value;
}

#[derive(Debug)]
pub enum Primitive {
    String(String),
    Integer(i64),
    Time(SystemTime),
    Empty,
}

impl Primitive {
    pub fn format(&self) -> String {
        match self {
            Primitive::String(s) => s.to_string(),
            Primitive::Integer(i) => i.to_string(),
            Primitive::Empty => todo!(),
            Primitive::Time(t) => {
                let as_utc: DateTime<Utc> = t.clone().into();
                as_utc.date_naive().to_string()
            }
        }
    }
}

#[derive(Debug)]
pub enum Value {
    Object(Box<dyn RshObject>),
    List(Vec<Value>),
    Primitive(Primitive),
}

impl Value {
    pub fn data_descriptors(&self) -> Vec<Descriptor> {
        match self {
            Value::Object(o) => o.data_descriptors(),
            Value::List(l) => todo!(),
            Value::Primitive(p) => todo!(),
        }
    }

    pub fn get_data(&self, desc: &Descriptor) -> &Value {
        match self {
            Value::Object(o) => o.get_data(desc),
            Value::List(l) => todo!(),
            Value::Primitive(p) => todo!(),
        }
    }

    pub fn format(&self) -> String {
        match self {
            Value::Object(o) => todo!(),
            Value::List(l) => todo!(),
            Value::Primitive(p) => p.format(),
        }
    }

    pub fn object(value: impl RshObject + 'static) -> Self {
        Value::Object(Box::new(value))
    }

    pub fn list(values: impl Into<Vec<Value>>) -> Self {
        Value::List(values.into())
    }

    pub fn string(string: impl Into<String>) -> Self {
        Value::Primitive(Primitive::String(string.into()))
    }

    pub fn int(int: impl Into<i64>) -> Self {
        Value::Primitive(Primitive::Integer(int.into()))
    }

    pub fn time(time: impl Into<SystemTime>) -> Self {
        Value::Primitive(Primitive::Time(time.into()))
    }

    pub fn empty() -> Self {
        Value::Primitive(Primitive::Empty)
    }
}

pub trait ToBaseView {
    fn to_base_view(&self) -> BaseView;
}

impl ToBaseView for Value {
    fn to_base_view(&self) -> BaseView {
        BaseView::new(self)
    }
}

pub trait Type: Debug {}

#[derive(Debug)]
pub struct Any;

impl Type for Any {}