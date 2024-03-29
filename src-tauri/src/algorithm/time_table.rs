pub mod cell;

use cell::ActiveCell;
use cell::BlankCell;
use cell::Cell;
use core::time;
use std::error::Error;
use std::sync::Mutex;

use crate::input::room;

use super::aco;
use super::aco::aco_solver::ACOSolver;
use super::aco::aco_solver::ACOSolverManager;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeTable {
    pub cells: Vec<Cell>,
}

pub fn convert_solver_to_timetable(solver: &ACOSolver) -> Result<TimeTable, Box<dyn Error>> {
    let mut cells = Vec::<Cell>::new();
    let best_ant = solver.get_best_ant().ok_or("No best ant found")?;
    for room in 0..solver.parameters.num_of_rooms {
        for period in 0..solver.parameters.num_of_periods {
            cells.push(Cell::BlankCell(BlankCell {
                id: (room * solver.parameters.num_of_periods + period) as usize,
                room: room as usize,
                period: period as usize,
                size: None,
            }));
        }
    }
    let classes = solver.input.get_classes().clone();
    for (class_id, &[room_id, period_id]) in best_ant.get_corresponding_crp().iter().enumerate() {
        let id = room_id * (solver.parameters.num_of_periods as usize) + period_id;
        cells[room_id as usize * solver.parameters.num_of_periods as usize + period_id as usize] =
            Cell::ActiveCell(ActiveCell {
                id: id as usize,
                period: period_id as usize,
                room: room_id as usize,
                class_index: class_id as usize,
                class_name: classes[class_id].get_name().clone(),
                teachers: None,
                students: None,
                color: Some(calc_color_init(solver, class_id, room_id, period_id)),
                is_locked: solver
                    .colony
                    .get_graph()
                    .get_classes_is_locked(class_id)
                    .map(|_| true),
                size: None,
            });
    }
    Ok(TimeTable { cells })
}

pub struct TimeTableManager {
    pub timetable_manager: Mutex<Option<TimeTable>>,
}

pub fn save_timetable(timetable_manager: tauri::State<'_, TimeTableManager>, timetable: TimeTable) {
    let mut managed_timetable = timetable_manager.timetable_manager.lock().unwrap();
    *managed_timetable = Some(timetable);
}

fn calc_color_init(
    solver: &ACOSolver,
    class_id: usize,
    room_id: usize,
    period_id: usize,
) -> String {
    let mut res = get_pheromone_color(solver, class_id, room_id, period_id);

    if let Some(is_lock) = solver.colony.get_graph().get_classes_is_locked(class_id) {
        if is_lock.0 == room_id as usize && is_lock.1 == period_id as usize {
            res = "#AAAAFF".to_string();
        }
    }
    return res;
}

fn get_pheromone_color(
    solver: &ACOSolver,
    class_id: usize,
    room_id: usize,
    period_id: usize,
) -> String {
    let mut res = String::from("#FFFFFF");
    if let Some(ant) = solver.get_best_ant() {
        let (rp_v, prov_v) =
            ant.calc_prob_from_v_igunore_visited(class_id, solver.colony.get_graph());

        let mut prov = 0.0;
        for (i, rp) in rp_v.iter().enumerate() {
            if rp[0] == room_id && rp[1] == period_id {
                prov = prov_v[i];
            }
        }
        let color = (255.0 - (prov * 255.0)) as u8;
        let hex = format!("{:02x}", color);
        res = format!("#ff{}{}ff", hex, hex);
    }
    res
}

fn calc_color_from_cell(solver: &ACOSolver, active_cell: &ActiveCell) -> String {
    if active_cell.is_locked.unwrap_or(false) {
        println!("::{}", active_cell.is_locked.unwrap_or(false));
        return "#AAAAFF".to_string();
    }
    let class_id = active_cell.class_index;
    let room_id = active_cell.room;
    let period_id = active_cell.period;
    return get_pheromone_color(solver, class_id, room_id, period_id);
}

#[tauri::command]
pub fn handle_lock_cell(
    timetable_manager: tauri::State<'_, TimeTableManager>,
    over_id: usize,
    active_id: usize,
) -> Result<TimeTable, String> {
    let mut managed_timetable = timetable_manager.timetable_manager.lock().unwrap();
    let mut new_timetable;
    if let Some(timetable) = managed_timetable.as_mut() {
        new_timetable = timetable.clone();
        if let Cell::ActiveCell(active_cell) = new_timetable.cells[active_id].clone() {
            if let Cell::BlankCell(blank_cell) = new_timetable.cells[over_id].clone() {
                new_timetable.cells[active_id] = Cell::BlankCell(blank_cell.clone());
                match &mut new_timetable.cells[active_id] {
                    Cell::BlankCell(blank_cell) => {
                        blank_cell.id = active_cell.id;
                        blank_cell.room = active_cell.room;
                        blank_cell.period = active_cell.period.clone();
                    }
                    _ => (),
                }
                new_timetable.cells[over_id] = Cell::ActiveCell(active_cell.clone());
                match &mut new_timetable.cells[over_id] {
                    Cell::ActiveCell(active_cell) => {
                        active_cell.id = blank_cell.id;
                        active_cell.room = blank_cell.room;
                        active_cell.period = blank_cell.period;
                    }
                    _ => (),
                }
                println!("Swaped cells");
                match &mut new_timetable.cells[over_id] {
                    Cell::ActiveCell(active_cell) => {
                        active_cell.is_locked = Some(true);
                        active_cell.color = Some("#AAAAFF".to_string());
                    }
                    _ => (),
                }
            }
        }
    } else {
        return Err("No timetable found".to_string());
    }
    *managed_timetable = Some(new_timetable.clone());
    return Ok(new_timetable);
}

#[tauri::command]
pub fn handle_switch_lock(
    timetable_manager: tauri::State<'_, TimeTableManager>,
    acosolver_manager: tauri::State<'_, ACOSolverManager>,
    id: usize,
) -> Result<TimeTable, String> {
    println!("called handle_switch_lock");
    let mut managed_timetable = timetable_manager.timetable_manager.lock().unwrap();
    let mut managed_solver = acosolver_manager.solver.lock().unwrap();
    if let Some(timetable) = managed_timetable.as_mut() {
        if let Some(solver) = managed_solver.as_mut() {
            if let Cell::ActiveCell(active_cell) = timetable.cells[id].as_mut() {
                active_cell.is_locked = Some(!active_cell.is_locked.unwrap_or(false));
                active_cell.color = Some(calc_color_from_cell(solver, active_cell));
                println!("{:?}", active_cell.is_locked.unwrap_or(false));
                println!(
                    "{:?}",
                    active_cell.color.clone().unwrap_or("#FFFFFF".to_string())
                );
            }
        }
        return Ok(timetable.clone());
    }
    return Err("No timetable found".to_string());
}
