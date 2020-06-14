use std::collections::{HashSet, HashMap};

use crate::database::{ComparisonId, get_components, get_packages, Opt, Package, PackageId, Version, Comparison, ComponentId};
use crate::portfolio::{City, components_in_cities, Region};
use crate::results::{CostCalculationSummaryLine, Scope};

// From the given packages, compute the set of ids of comparisons that have been
// replaced by other comparisons in the given packages. Assume there are no cyclic
// dependencies, i.e. comparisons replace each other mutually or transitively.
fn get_replaced_comparsion_ids<'a>(packages: &[&'a Package]) -> HashSet<&'a ComparisonId> {
    return packages
        .iter()
        .flat_map(|pkg| &pkg.comparisons)
        .flat_map(|comparison| &comparison.replacing)
        .collect();
}

// Given a list of packages, the version and a set of replaced comparisonIds,
// extract a list of pairs (one per comparison) with the first element representing
// the nullable "current" option and the second element the non-nullable
// selected (in the version) element.
fn get_existing_and_selected_options<'a>(
    packages: &[&'a Package],
    version: &Version,
    replaced_comparisons: HashSet<&ComparisonId>,
) -> Vec<(Option<&'a Opt>, &'a Opt)> {
    let mut result: Vec<(Option<&'a Opt>, &'a Opt)> = vec![];
    packages
        .iter()
        .flat_map(|package| &package.comparisons)
        .for_each(|comparison|
            if !replaced_comparisons.contains(&comparison.id) {
                let current = &comparison.options.iter().filter(|opt| opt.existing).collect::<Vec<_>>()[0];
                let n = &version.selections
                    .iter()
                    .map(|selection| &selection.option_id)
                    .collect::<Vec<_>>()[0];
                let new = &comparison.options.iter().filter(|v| &v.id == *n).collect::<Vec<_>>()[0];
                result.push((Some(*current), *new))
            }
        );

    return result;
}

// Given an option and a city, compute the sum price of this option's
// components in that city, taking into account the number of components of this
// category in that city.
fn compute_costs_for_option_in_city(
    option: &Opt,
    city: &City,
) -> f32 {
    let componentsIds = &option
        .component_refs
        .iter()
        .map(|cr| &cr.component_id)
        .collect::<Vec<_>>();

    return get_components(componentsIds)
        .iter()
        .map(|component| {
            *(components_in_cities(&component.category).get(&city).unwrap()) * component.price
        }).sum();
}

// Given a list of packageIds and a version, calculate the cost summary lines for
// the group, regions and cities. The group line should come first, then the regions
// with their respective cities, e.g.:
// group
// scandinavia
// stockholm
// malmo
// europe
// berlin
// ...
//
// Assume that every version passed here will be complete, i.e. for every comparison there
// is a selection in the version.
//
// Note: Rounding is not a part of this problem. Just convert floating point numbers to integers.
#[allow(unused)]
fn calculate_summary_lines(
    package_ids: &[&PackageId],
    version: &Version,
) -> Vec<CostCalculationSummaryLine> {
    let packages = get_packages(package_ids);

    let options = get_existing_and_selected_options(
        &packages,
        &version,
        get_replaced_comparsion_ids(&packages),
    );

    println!("{:?}", &options);

    return vec![];
}

#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;
    use maplit::hashset;

    use crate::database::{get_packages, get_version, OptionId, PackageId, VersionId};
    use crate::portfolio::Region;
    use crate::results::Scope;

    use super::*;

    #[test]
    fn should_compute_replaced_comparison_ids() {
        let pkg_id1 = PackageId("package-1".into());
        let pkg_id2 = PackageId("package-2".into());
        let values = vec![
            (vec![&pkg_id1], HashSet::new()),
            (vec![&pkg_id2], hashset! { ComparisonId("comparison-1".into()) }),
        ];

        for (input, expectation) in values {
            let expectation = expectation.iter().map(|it| it).collect();

            let packages = get_packages(&input);
            let result = get_replaced_comparsion_ids(&packages);
            assert_eq!(result, expectation);
        }
    }

    #[test]
    fn should_extract_the_list_of_current_and_selected_options() {
        let pkg_id1 = PackageId("package-1".into());
        let pkg_ids = vec![&pkg_id1];
        let pkg = get_packages(&pkg_ids);
        let comparison_id = ComparisonId("comparison-2".into());
        let result = get_existing_and_selected_options(
            &pkg,
            get_version(&VersionId("v1".into())).unwrap(),
            hashset! { &comparison_id },
        );

        let comparison = pkg[0].comparisons.iter()
            .find(|c| c.id == ComparisonId("comparison-1".into()))
            .unwrap();
        let opt1 = comparison.options.iter()
            .find(|o| o.id == OptionId("door-1".into()))
            .unwrap();
        let opt2 = comparison.options.iter()
            .find(|o| o.id == OptionId("door-2".into()))
            .unwrap();

        assert_eq!(result, &[(Some(opt1), opt2)]);
    }

    #[test]
    fn should_compute_the_price_of_an_option_with_a_single_component_in_a_city() {
        let ids = &[&PackageId("package-1".into())];
        let opt = get_packages(ids)[0]
            .comparisons.iter()
            .find(|it| &it.id.0 == "comparison-1").unwrap()
            .options.iter().find(|it| &it.id.0 == "door-1").unwrap();
        let result = compute_costs_for_option_in_city(
            opt,
            &City::Berlin,
        );

        assert!(approx_eq!(f32, result, 150.0, epsilon = 0.00001));
    }

    #[test]
    fn should_compute_the_price_of_an_option_with_multiple_component_in_a_city() {
        let ids = &[&PackageId("package-3".into())];
        let opt = get_packages(ids)[0]
            .comparisons.iter()
            .find(|it| &it.id.0 == "comparison-4").unwrap()
            .options.iter().find(|it| &it.id.0 == "tiles-2").unwrap();
        let result = compute_costs_for_option_in_city(
            opt,
            &City::Stockholm,
        );

        assert!(approx_eq!(f32, result, 115.2, epsilon = 0.00001));
    }

    #[test]
    fn should_calculate_cost_summary_lines_with_package_1_and_v1() {
        let result = calculate_summary_lines(
            &[
                &PackageId("package-1".into()),
            ],
            get_version(&VersionId("v1".into())).unwrap(),
        );

        let expectation = vec![
            CostCalculationSummaryLine { scope: Scope::Group, existing_cost: 162, selected_cost: 172 },
            CostCalculationSummaryLine { scope: Scope::Region(Region::Scandinavia), existing_cost: 256, selected_cost: 272 },
            CostCalculationSummaryLine { scope: Scope::City(City::Stockholm), existing_cost: 308, selected_cost: 328 },
            CostCalculationSummaryLine { scope: Scope::City(City::Malmo), existing_cost: 247, selected_cost: 262 },
            CostCalculationSummaryLine { scope: Scope::Region(Region::Europe), existing_cost: 201, selected_cost: 213 },
            CostCalculationSummaryLine { scope: Scope::City(City::Berlin), existing_cost: 244, selected_cost: 259 },
            CostCalculationSummaryLine { scope: Scope::City(City::Hamburg), existing_cost: 234, selected_cost: 249 },
            CostCalculationSummaryLine { scope: Scope::City(City::Munich), existing_cost: 232, selected_cost: 247 },
        ];

        assert_eq!(result, expectation);
    }

    #[test]
    fn should_calculate_cost_summary_lines_with_all_packages_and_v1() {
        let result = calculate_summary_lines(
            &[
                &PackageId("package-1".into()),
                &PackageId("package-2".into()),
                &PackageId("package-3".into()),
            ],
            get_version(&VersionId("v1".into())).unwrap(),
        );

        let expectation = vec![
            CostCalculationSummaryLine { scope: Scope::Group, existing_cost: 0, selected_cost: 248 },
            CostCalculationSummaryLine { scope: Scope::Region(Region::Scandinavia), existing_cost: 0, selected_cost: 393 },
            CostCalculationSummaryLine { scope: Scope::City(City::Stockholm), existing_cost: 0, selected_cost: 475 },
            CostCalculationSummaryLine { scope: Scope::City(City::Malmo), existing_cost: 0, selected_cost: 374 },
            CostCalculationSummaryLine { scope: Scope::Region(Region::Europe), existing_cost: 0, selected_cost: 306 },
            CostCalculationSummaryLine { scope: Scope::City(City::Berlin), existing_cost: 0, selected_cost: 370 },
            CostCalculationSummaryLine { scope: Scope::City(City::Hamburg), existing_cost: 0, selected_cost: 359 },
            CostCalculationSummaryLine { scope: Scope::City(City::Munich), existing_cost: 0, selected_cost: 358 },
        ];

        assert_eq!(result, expectation);
    }
}
