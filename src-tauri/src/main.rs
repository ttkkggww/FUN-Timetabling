// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use tauri::Manager;
mod algorithm;
mod input;
use std::error::Error;
mod table_editor;
use algorithm::aco::aco_parameters::AcoParametersManager;
use algorithm::aco::aco_solver::ACOSolverManager;
use algorithm::time_table;
use input::InputManager;
use std::time::Instant;

#[tauri::command]
fn handle_adapt_input(
    input_manager: tauri::State<'_, InputManager>,
    solver_manager: tauri::State<'_, ACOSolverManager>,
    aco_parameters_manager: tauri::State<'_, AcoParametersManager>,
) -> Result<(), String> {
    let input = input_manager.input.lock().unwrap();
    if let Some(input) = input.clone() {
        println!("adapt input to solver.");
        let parameters = algorithm::aco::aco_parameters::AcoParameters {
            num_of_ants: 3,
            num_of_classes: input.get_classes().len(),
            num_of_rooms: input.get_rooms().len(),
            num_of_periods: 5 * 6 * 4,
            num_of_day_lengths: 4,
            num_of_teachers: input.get_teachers().len(),
            num_of_students: input.get_student_groups().len(),
            size_of_frame: 4,
            q: 10.0,
            alpha: 1.0,
            beta: 1.0,
            rou: 0.5,
            max_iterations: 100,
            tau_min: 0.001,
            tau_max: 100000.0,
            ant_prob_random: 0.0,
            super_not_change: 10000,
        };
        let solver = Some(algorithm::aco::aco_solver::ACOSolver {
            parameters: parameters.clone(),
            colony: algorithm::aco::colony::Colony::new(
                algorithm::aco::graph::Graph::new(
                    parameters.clone(),
                    input.get_classes().clone(),
                    input.get_rooms().clone(),
                    input.get_teachers().clone(),
                ),
                parameters.clone(),
            ),
            best_ant: None,
            super_ant: None,
            cnt_super_not_change: 0,
            input: input,
        });
        let mut manarged_solver = solver_manager.solver.lock().unwrap();
        manarged_solver.replace(solver.unwrap());
        let mut managed_parameters = aco_parameters_manager.parameters.lock().unwrap();
        managed_parameters.replace(parameters);
    } else {
        println!("no input!");
    }
    Ok(())
}
use input::handle_set_input;

#[tauri::command]
fn handle_aco_run_once(
    solver_manager: tauri::State<'_, ACOSolverManager>,
    timetable_manager: tauri::State<'_, time_table::TimeTableManager>,
) -> Result<time_table::TimeTable, String> {
    let mut managed_solver = solver_manager.solver.lock().unwrap();

    if let Some(solver) = managed_solver.as_mut() {
        let mut run_cnt = 0;
        let start = Instant::now();
        for _ in 0..10000 {
            solver.run_aco_times(1);
            run_cnt += 1;
            if let Some(best_ant) = &solver.best_ant {
                println!(
                    "{:?}",
                    best_ant.calc_all_path_length(solver.colony.get_graph())
                );
                if best_ant.calc_all_path_length(solver.colony.get_graph()) <= 1.5 {
                    break;
                }
            }
        }
        let duaration = start.elapsed();
        println!("times:{:?},{:?}", run_cnt, duaration);
        let res = time_table::convert_solver_to_timetable(solver).map_err(|e| e.to_string())?;
        time_table::save_timetable(timetable_manager, res.clone());
        /*
        println!(
            "violations_strict_student{:?}",
            solver.get_best_ant_same_group_violations_strictly()
        );
        println!(
            "violations_strict_teacher{:?}",
            solver.get_best_ant_same_teacher_violations_strictly()
        );
        println!(
            "violations_capacity{:?}",
            solver.get_best_ant_capacity_violations()
        );
        println!(
            "violations_strabble_days{:?}",
            solver.get_best_ant_strabble_days_violations()
        );
        */
        return Ok(res);
    }
    return Err("No ACOSolver".to_string());
}

use algorithm::aco::aco_parameters::handle_get_periods;
use algorithm::aco::aco_solver::handle_one_hot_pheromone;
use algorithm::aco::aco_solver::handle_read_cells;
use input::handle_get_rooms;
use table_editor::handle_get_table;
use time_table::handle_swap_cell;
use time_table::handle_switch_lock;
use time_table::is_swappable;

fn main() -> Result<(), Box<dyn Error>> {
    //let input = input::Input::new();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            handle_adapt_input,
            handle_set_input,
            handle_aco_run_once,
            handle_one_hot_pheromone,
            handle_get_table,
            handle_swap_cell,
            handle_read_cells,
            handle_switch_lock,
            is_swappable,
            handle_get_periods,
            handle_get_rooms
        ])
        .setup(|app| {
            let input_manager = InputManager {
                input: Mutex::new(None),
            };
            app.manage(input_manager);
            let solver_manager = ACOSolverManager {
                solver: Mutex::new(None),
            };
            app.manage(solver_manager);
            let timetable_manager = time_table::TimeTableManager {
                timetable_manager: Mutex::new(None),
            };
            app.manage(timetable_manager);
            let aco_parameters_manager = AcoParametersManager {
                parameters: Mutex::new(None),
            };
            app.manage(aco_parameters_manager);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
