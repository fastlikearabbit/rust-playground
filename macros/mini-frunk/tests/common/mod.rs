use mini_frunk_core::{ hlist::HCons, hlist::HNil, HList, hlist, hlist_pat };
use mini_frunk_core::generic::Generic;
use mini_frunk_derive::Generic;

#[derive(Generic, Debug, Clone, PartialEq)]
pub struct Person<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub age: usize,
}

#[derive(Generic, Debug)]
pub struct JumbledPerson<'a> {
    pub last_name: &'a str,
    pub first_name: &'a str,
    pub age: usize,
}

#[derive(Generic)]
pub struct LongPerson<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub bank_title: &'a str,
    pub account_balance: f64,
}

#[derive(Generic)]
pub struct BankAccount<'a> {
    pub bank_title: &'a str,
    pub account_balance: f64,
}

#[derive(Generic)]
pub struct ShortPerson<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
}

#[derive(Generic, Debug, PartialEq, Clone)]
pub struct Strategist<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub age: usize,
}

#[derive(Generic, Debug, PartialEq)]
pub struct President<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub age: usize,
}

#[derive(Generic, Debug, PartialEq, Clone)]
pub struct SavedUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub age: usize,
}

#[allow(non_snake_case)]
#[derive(Generic, Debug)]
pub struct ApiUser<'a> {
    pub FirstName: &'a str,
    pub LastName: &'a str,
    pub Age: usize,
}
