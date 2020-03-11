use std::collections::HashMap;

use maplit::hashmap;

use crate::database::Category;

pub fn components_in_cities(cat: &Category) -> HashMap<City, f32> {
    match cat {
        Category::Door => hashmap! {
            City::Stockholm => 2.0,
            City::Malmo => 1.5,
            City::Berlin => 1.5,
            City::Hamburg => 1.5,
            City::Munich => 1.5,
        },
        Category::WallTile => hashmap! {
            City::Stockholm => 7.2,
            City::Malmo => 6.5,
            City::Berlin => 6.3,
            City::Hamburg => 5.6,
            City::Munich => 5.5,
        }
    }
}

#[allow(unused)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Region {
    Scandinavia,
    Europe,
}

impl Region {
    pub fn cities(&self) -> &[City] {
        match &self {
            Region::Scandinavia => &[
                City::Stockholm,
                City::Malmo,
            ],
            Region::Europe => &[
                City::Berlin,
                City::Hamburg,
                City::Munich,
            ]
        }
    }

    pub fn values() -> &'static[Region] {
        lazy_static! {
            static ref VARIANTS: Vec<Region> = vec![Region::Scandinavia, Region::Europe];
        }

        &VARIANTS
    }
}

#[allow(unused)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum City {
    Stockholm,
    Malmo,
    Berlin,
    Hamburg,
    Munich,
}

impl City {
    pub fn weight_to_region(&self) -> f32 {
        match &self {
            City::Stockholm => 0.5370857921016795,
            City::Malmo => 0.3693145710394916,
            City::Berlin => 0.6154622438681936,
            City::Hamburg => 0.1907770611206178,
            City::Munich => 0.02755473651880128,
        }
    }

    pub fn weight_to_group(&self) -> f32 {
        match &self {
            City::Stockholm => 0.12980516061084782,
            City::Malmo => 0.08925750394944708,
            City::Berlin => 0.3077716341934351,
            City::Hamburg => 0.09540108829208355,
            City::Munich => 0.013779182025627524,
        }
    }
}
