#[derive(Debug, Eq, PartialEq)]
pub struct PackageId(pub String);

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct ComparisonId(pub String);

#[derive(Debug, Eq, PartialEq)]
pub struct OptionId(pub String);

#[derive(Debug, Eq, PartialEq)]
pub struct ComponentId(pub String);

#[derive(Debug, Eq, PartialEq)]
pub struct VersionId(pub String);

pub fn get_packages<'a, 'b>(ids: &[&'a PackageId]) -> Vec<&'b Package> {
    PACKAGES.iter()
        .filter(|p| ids.contains(&&p.id))
        .collect()
}

pub fn get_components<'a, 'b>(ids: &[&'a ComponentId]) -> Vec<&'b Component> {
    COMPONENTS.iter()
        .filter(|c| ids.contains(&&c.id))
        .collect()
}

#[allow(unused)]
pub fn get_version(id: &VersionId) -> Option<&Version> {
    VERSIONS.iter()
        .find(|v| v.id == *id)
}

lazy_static!{
    static ref VERSIONS: Vec<Version> = vec![
        Version {
            id: VersionId("v1".into()),
            selections: vec![
                Selection {
                    package_id: PackageId("package-1".into()),
                    comparison_id: ComparisonId("comparison-1".into()),
                    option_id: OptionId("door-2".into()),
                },
                Selection {
                    package_id: PackageId("package-1".into()),
                    comparison_id: ComparisonId("comparison-2".into()),
                    option_id: OptionId("tiles-1".into()),
                },
                Selection {
                    package_id: PackageId("package-2".into()),
                    comparison_id: ComparisonId("comparison-3".into()),
                    option_id: OptionId("door-3".into()),
                },
                Selection {
                    package_id: PackageId("package-3".into()),
                    comparison_id: ComparisonId("comparison-4".into()),
                    option_id: OptionId("tiles-2".into()),
                },
            ],
        }
    ];
}

lazy_static! {
    static ref PACKAGES: Vec<Package> = vec![
        Package {
            id: PackageId("package-1".into()),
            title: "BASE".into(),
            comparisons: vec![
                Comparison {
                    id: ComparisonId("comparison-1".into()),
                    title: "Doors".into(),
                    options: vec![
                        Opt {
                            id: OptionId("door-1".into()),
                            title: "white door".into(),
                            existing: true,
                            component_refs: vec![
                                ComponentRef {
                                    component_id: ComponentId("cr-door-1".into())
                                }
                            ]
                        },
                        Opt {
                            id: OptionId("door-2".into()),
                            title: "gray door".into(),
                            existing: false,
                            component_refs: vec![
                                ComponentRef {
                                    component_id: ComponentId("cr-door-2".into())
                                }
                            ]
                        }
                    ],
                    replacing: vec![],
                },
                Comparison {
                    id: ComparisonId("comparison-2".into()),
                    title: "Wall tiles".into(),
                    options: vec![
                        Opt {
                            id: OptionId("tiles-1".into()),
                            title: "white tiles".into(),
                            existing: true,
                            component_refs: vec![
                                ComponentRef {
                                    component_id: ComponentId("cr-tile-1".into())
                                }
                            ]
                        }
                    ],
                    replacing: vec![],
                }
            ]
        },
        Package {
            id: PackageId("package-2".into()),
            title: "FANCYDOORS".into(),
            comparisons: vec![
                Comparison {
                    id: ComparisonId("comparison-3".into()),
                    title: "fancy doors".into(),
                    options: vec![
                        Opt {
                            id: OptionId("door-3".into()),
                            title: "mirror door".into(),
                            existing: false,
                            component_refs: vec![
                                ComponentRef {
                                    component_id: ComponentId("cr-door-3".into())
                                }
                            ]
                        }
                    ],
                    replacing: vec![ComparisonId("comparison-1".into())]
                }
            ]
        },
        Package {
            id: PackageId("package-3".into()),
            title: "FANCYTILES".into(),
            comparisons: vec![
                Comparison {
                    id: ComparisonId("comparison-4".into()),
                    title: "mosaic tiles".into(),
                    options: vec![
                        Opt {
                            id: OptionId("tiles-2".into()),
                            title: "mosaic tiles".into(),
                            existing: false,
                            component_refs: vec![
                                ComponentRef {
                                    component_id: ComponentId("cr-tile-2".into())
                                },
                                ComponentRef {
                                    component_id: ComponentId("cr-tile-3".into())
                                }
                            ]
                        }
                    ],
                    replacing: vec![ComparisonId("comparison-2".into())]
                }
            ]
        }
    ];
}

lazy_static! {
    static ref COMPONENTS: Vec<Component> = vec![
        Component { id: ComponentId("cr-door-1".into()), price: 100.0, category: Category::Door },
        Component { id: ComponentId("cr-door-2".into()), price: 110.0, category: Category::Door },
        Component { id: ComponentId("cr-door-3".into()), price: 180.0, category: Category::Door },
        Component { id: ComponentId("cr-tile-1".into()), price:  15.0, category: Category::WallTile },
        Component { id: ComponentId("cr-tile-2".into()), price:   9.0, category: Category::WallTile },
        Component { id: ComponentId("cr-tile-3".into()), price:   7.0, category: Category::WallTile }
    ];
}

#[derive(Debug)]
pub struct Package {
    pub id: PackageId,
    pub title: String,
    pub comparisons: Vec<Comparison>,
}

#[derive(Debug)]
pub struct Comparison {
    pub id: ComparisonId,
    pub title: String,
    pub options: Vec<Opt>,
    pub replacing: Vec<ComparisonId>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Opt {
    pub id: OptionId,
    pub title: String,
    pub existing: bool,
    pub component_refs: Vec<ComponentRef>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ComponentRef {
    pub component_id: ComponentId,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Version {
    pub id: VersionId,
    pub selections: Vec<Selection>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Selection {
    pub package_id: PackageId,
    pub comparison_id: ComparisonId,
    pub option_id: OptionId,
}

#[derive(Debug)]
pub struct Component {
    pub id: ComponentId,
    pub price: f32,
    pub category: Category,
}

#[derive(Debug)]
pub enum Category {
    Door,
    WallTile,
}
