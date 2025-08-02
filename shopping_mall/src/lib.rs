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
        .flat_map(|floor| floor.stores.values())
        .map(|store| store.employees.len())
        .sum()
}

pub fn check_for_securities(mall: &mut Mall, available_guards: HashMap<String, Guard>) {
    let total_size: u64 = mall.floors
        .values()
        .flat_map(|floor| floor.stores.values())
        .map(|store| store.square_meters)
        .sum();

    let required_guards = (total_size / 200) as usize;
    let current_guards = mall.guards.len();

    if current_guards < required_guards {
        let needed = required_guards - current_guards;
        for (name, guard) in available_guards.into_iter().take(needed) {
            mall.hire_guard(name, guard);
        }
    }
}

pub fn cut_or_raise(mall: &mut Mall) {
    for floor in mall.floors.values_mut() {
        for store in floor.stores.values_mut() {
            for employee in store.employees.values_mut() {
                let hours = employee.working_hours.1 - employee.working_hours.0;
                if hours >= 10 {
                    employee.salary *= 1.1;
                } else {
                    employee.salary *= 0.9;
                }
                employee.salary = (employee.salary * 1000.0).round() / 1000.0;
            }
        }
    }
}
