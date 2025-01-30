use std::collections::{HashMap, HashSet};

use crate::utils;

struct Company {
    employee_department: HashMap<String, HashSet<String>>,
    department_employee: HashMap<String, HashSet<String>>,
}

impl Company {
    fn new() -> Company {
        Company {
            employee_department: HashMap::new(),
            department_employee: HashMap::new(),
        }
    }

    /// Add an employee to the company. Returns true if the employee was added successfully, false
    /// if the employee is already in the department.
    fn add_employee(&mut self, name: &str, department: &str) -> bool {
        let dept_already_in_empl = self
            .employee_department
            .entry(name.to_string())
            .or_default()
            .insert(department.to_string());
        let empl_already_in_dept = self
            .department_employee
            .entry(department.to_string())
            .or_default()
            .insert(name.to_string());

        dept_already_in_empl && empl_already_in_dept
    }
}

pub fn launch() {
    utils::clear_screen();
    let mut company = Company::new();
    println!("Add or see employees to your company!");
    loop {
        utils::clear_screen();
        println!("1. Add a new employee");
        println!("2. Search the company");
        println!("0. Back");

        let selection = utils::read_input();

        let selection = match selection.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid selection. Please enter a valid number.");
                continue;
            }
        };

        match selection {
            1 => {
                utils::clear_screen();
                println!("Enter employee name:");
                let name = utils::read_input();
                if name.trim().is_empty() {
                    println!("Employee name cannot be empty");
                    continue;
                }
                println!("Enter department:");
                let department = utils::read_input();
                if department.trim().is_empty() {
                    println!("Department name cannot be empty");
                    continue;
                }
                let employee_already_exist = company.add_employee(name.trim(), department.trim());
                if employee_already_exist {
                    println!("Employee already exists in the department");
                } else {
                    println!("Employee added successfully to the department");
                }
            }
            2 => loop {
                utils::clear_screen();
                println!("1. View all employees");
                println!("2. View employees by department");
                println!("3. View departments by employee");
                println!("0. Back");

                let selection = utils::read_input();
                let selection = match selection.trim().parse::<u32>() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid selection. Please enter a valid number.");
                        continue;
                    }
                };

                match selection {
                    1 => {
                        utils::clear_screen();
                        if company.employee_department.is_empty() {
                            println!("No employees in the company");
                        } else {
                            for (department, employees) in &company.department_employee {
                                println!("Department: {}", department);
                                println!("{}", "=".repeat(20));
                                for (i, employee) in employees.iter().enumerate() {
                                    println!("{}. {}", i, employee);
                                }
                                println!();
                            }
                        }
                    }
                    2 => {
                        utils::clear_screen();
                        println!("Enter department:");
                        let department = utils::read_input();
                        match company.department_employee.get(&department) {
                            Some(employees) => {
                                for (i, employee) in employees.iter().enumerate() {
                                    println!("{}. {}", i, employee);
                                }
                            }
                            None => println!("Department not found"),
                        }
                    }
                    3 => {
                        utils::clear_screen();
                        println!("Enter employee name:");
                        let name = utils::read_input();
                        match company.employee_department.get(&name) {
                            Some(departments) => {
                                for (i, department) in departments.iter().enumerate() {
                                    println!("{}. {}", i, department);
                                }
                            }
                            None => println!("Employee not found"),
                        }
                    }
                    0 => break,
                    _ => {
                        println!("Invalid selection");
                        continue;
                    }
                }

                println!("Press enter to continue...");
                utils::read_input();
            },
            0 => break,
            _ => {
                println!("Invalid selection");
                continue;
            }
        }
    }
}
