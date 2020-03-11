use crate::portfolio::{Region, City};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct CostCalculationSummaryLine {
    pub scope: Scope,
    pub existing_cost: i32,
    pub selected_cost: i32,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Scope {
    Group,
    Region(Region),
    City(City),
}
