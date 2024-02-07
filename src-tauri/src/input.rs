use serde::{Deserialize, Serialize};

use std::{error::Error, io , process};

use self::{student_group::StudentGroup, teacher::Teacher};

pub mod class;
pub mod room;
mod student_group;
mod teacher;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Input {
    classes: Vec<class::Class>,
    rooms: Vec<room::Room>,
    student_groups: Vec<student_group::StudentGroup>,
    teachers: Vec<teacher::Teacher>,
}

const TEACHERS_CSV_PATH : &str = "./csvdata/teachers.csv";
const STUDENT_GROUPS_CSV_PATH : &str = "./csvdata/teachers.csv";
const CLASSES_CSV_PATH : &str = "./csvdata/classes.csv";
const ROOMS_CSV_PATH : &str = "./csvdata/rooms.csv";

impl Input{
    fn new () -> Input{
        let teachers = Input::read_teachers_from_csv(&TEACHERS_CSV_PATH.to_string()).unwrap();
        let rooms = Input::read_rooms_from_csv(&ROOMS_CSV_PATH.to_string()).unwrap();
        let student_groups = Input::read_student_groups_from_csv(&STUDENT_GROUPS_CSV_PATH.to_string()).unwrap();
        let classes = Input::read_classes_from_csv(&CLASSES_CSV_PATH.to_string(),&teachers,&rooms,&student_groups).unwrap();
        Input{classes,rooms,student_groups,teachers}

    }

    fn read_teachers_from_csv(file_path:&String) -> Result<Vec<teacher::Teacher>,Box<dyn Error>> {
        let mut rdr = csv::Reader::from_path(file_path)?;
        let mut teachers = Vec::new();
        for (index,result) in rdr.records().enumerate() {
            let record = result?;
            let id = record[0].parse::<u64>().unwrap();
            let name = record[1].to_string();
            let index = index as u64;
            teachers.push(teacher::Teacher{id,index,name});
        }
        Ok(teachers)
    }

    fn read_rooms_from_csv(file_path:&String) -> Result<Vec<room::Room>,Box<dyn Error>> {
        let mut rdr = csv::Reader::from_path(file_path)?;
        let mut rooms = Vec::new();
        for (index,result) in rdr.records().enumerate() {
            let record = result?;
            let index = index as u64;
            let id = record[0].parse::<u64>().unwrap();
            let name = record[1].to_string();
            let capacity = record[2].parse::<u64>().unwrap();
            rooms.push(room::Room{id,index,name,capacity});
        }
        Ok(rooms)
    }

    fn read_student_groups_from_csv(file_path:&String) -> Result<Vec<student_group::StudentGroup>,Box<dyn Error>> {
        let mut rdr = csv::Reader::from_path(file_path)?;
        let mut student_groups = Vec::new();
        for result in rdr.records() {
            let record = result?;
            let id = record[0].parse::<u64>().unwrap();
            let index = record[1].parse::<u64>().unwrap();
            let name = record[2].to_string();
            student_groups.push(student_group::StudentGroup{id,index,name});
        }
        Ok(student_groups)
    }

    fn read_classes_from_csv(file_path:&String,teachers:&Vec<Teacher>,rooms:&Vec<room::Room>,student_groups:&Vec<StudentGroup>) -> Result<Vec<class::Class>,Box<dyn Error>> {
        let mut rdr = csv::Reader::from_path(file_path)?;
        let mut classes = Vec::new();
        for (index ,result) in rdr.records().enumerate() {
            let record = result?;
            let index = index as u64;
            let id = record[0].parse::<u64>().unwrap();
            let name = record[1].to_string();
            let mut teacher_indexes = Vec::new();
            for i in record[2].split(","){
                if let Some(add) = teachers.iter().position(|x| x.name== i){
                    teacher_indexes.push(add as u64);
                }else {
                    panic!("teacher not found");
                }
            }
            let mut room_candidates_indexes = Vec::new();
            for i in record[3].split(","){
                if let Some(add) = rooms.iter().position(|x| x.name== i){
                    room_candidates_indexes.push(add as u64);
                }else {
                    panic!("room not found");
                }
            }
            let mut students_group_indexes = Vec::new();
            for i in record[4].split(","){
                if let Some(add) = student_groups.iter().position(|x| x.name== i){
                    students_group_indexes.push(add as u64);
                }else {
                    panic!("student_group not found");
                }
            }
            let num_of_students = record[5].parse::<u64>().unwrap();
            classes.push(class::Class{id,index,num_of_students,name,teacher_indexes,room_candidates_indexes,students_group_indexes});
        }
        Ok(classes)
    }
    

    pub fn get_classes(&self) -> &Vec<class::Class>{
        &self.classes
    }
    pub fn get_rooms(&self) -> &Vec<room::Room>{
        &self.rooms
    }
    pub fn get_student_groups(&self) -> &Vec<student_group::StudentGroup>{
        &self.student_groups
    }
    pub fn get_teachers(&self) -> &Vec<teacher::Teacher>{
        &self.teachers
    }

    fn read_csv(file_path:&String) -> Result<(),Box<dyn Error>> {
        let mut rdr = csv::Reader::from_path(file_path)?;
        for result in rdr.records() {
            let record = result?;
            println!("{:?}", record);
        }
        Ok(())
    }
    
}