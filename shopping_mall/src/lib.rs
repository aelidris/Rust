pub mod mall;
pub use mall::*;
use std::collections::HashMap;
pub fn biggest_store(mall: &Mall) -> (&String, &Store) {
    mall.floors
        .values()
        .flat_map(|floor| floor.stores.iter())
        .max_by_key(|(_, store)| store.square_meters)
        .unwrap()
}

pub fn highest_paid_employee(mall: &Mall) -> Vec<(&String, &Employee)> {
    let max_salary = mall.floors
        .values()
        .flat_map(|floor| floor.stores.values())
        .flat_map(|store| store.employees.values())
        .map(|emp| emp.salary)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(0.0);

    mall.floors
        .values()
        .flat_map(|floor| floor.stores.iter())
        .flat_map(|(_store_name, store)| {
            store.employees
                .iter()
                .filter(move |(_, emp)| emp.salary == max_salary)
                .map(move |(emp_name, emp)| (emp_name, emp))
        })
        .collect()
}

pub fn nbr_of_employees(mall: &Mall) -> usize {
    mall.floors
        .values()
        .flat_map(|f| f.stores.values().flat_map(|s| &s.employees))
        .count() + mall.guards.len()
}

pub fn check_for_securities(mall: &mut Mall, available_guards: HashMap<String, Guard>) {
    let total_size = mall.floors
        .values()
        .map(|f| f.size_limit)
        .sum::<u64>();

    let total_areas = total_size / 200;
    let unguarded_areas = (total_areas as usize) - mall.guards.len();

    available_guards
        .into_iter()
        .take(unguarded_areas)
        .for_each(|(name, guard)| {
            mall.hire_guard(name, guard);
        });
}

pub fn cut_or_raise(mall: &mut Mall) {
    mall.floors
        .values_mut()
        .flat_map(|f| f.stores.values_mut().flat_map(|s| s.employees.values_mut()))
        .for_each(|e| {
            let shift_hours = e.working_hours.1 - e.working_hours.0;
            if shift_hours >= 10 {
                e.raise(e.salary * 0.1);
            } else {
                e.cut(e.salary * 0.1);
            }
        });
}
