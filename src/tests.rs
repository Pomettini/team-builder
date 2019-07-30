#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

extern crate float_cmp;

use crate::tests::float_cmp::*;

use super::*;

static TEST_FILE_EVEN: &str = "resources/test_even.csv";
static TEST_FILE_UNEVEN: &str = "resources/test_uneven.csv";
static TEST_FILE_WRONG: &str = "resources/test.csv";
static TEST_FILE_SAME_VALUES: &str = "resources/test_same_values.csv";

// TODO: Add tests for same values

const SKILL_GAME_DESIGN: usize = 0;
const SKILL_LEVEL_DESIGN: usize = 1;
const SKILL_PROGRAMMING: usize = 2;
const SKILL_NARRATIVE: usize = 3;
const SKILL_GRAPHICS: usize = 4;
const SKILL_TEAMWORK: usize = 5;

#[allow(unused_macros)]
macro_rules! SETUP_TEAMBUILDER_TEST {
    ($file:expr, $path:ident, $tb:ident) => {
        let $path = Path::new($file);
        #[allow(unused_mut)]
        let mut $tb = TeamBuilder::load_file(&$path).expect("File not found");
    };
}

#[allow(unused_macros)]
macro_rules! SETUP_TEAMBUILDER_TEST_AND_INIT {
    ($file:expr, $path:ident, $tb:ident) => {
        SETUP_TEAMBUILDER_TEST!($file, $path, $tb);
        $tb.process_file().expect("Cannot process file");
    };
}

#[test]
fn test_load_csv_correct_path() {
    SETUP_TEAMBUILDER_TEST!(TEST_FILE_EVEN, path, tb);
    assert!(!tb.students_file.is_empty());
}

#[test]
#[should_panic]
fn test_load_csv_wrong_path() {
    SETUP_TEAMBUILDER_TEST!(TEST_FILE_WRONG, path, tb);
    assert!(!tb.students_file.is_empty());
}

#[test]
fn test_load_csv_valid_content() {
    SETUP_TEAMBUILDER_TEST_AND_INIT!(TEST_FILE_EVEN, path, tb);
    assert!(!tb.students.is_empty());
}

#[test]
#[should_panic]
fn test_load_csv_not_valid_content() {
    SETUP_TEAMBUILDER_TEST_AND_INIT!(TEST_FILE_WRONG, path, tb);
    assert!(!tb.students.is_empty());
}

#[test]
fn test_get_students_number_green() {
    SETUP_TEAMBUILDER_TEST_AND_INIT!(TEST_FILE_EVEN, path, tb);
    assert_eq!(tb.students.len(), 6);
}

#[test]
#[should_panic]
fn test_get_students_number_red() {
    SETUP_TEAMBUILDER_TEST_AND_INIT!(TEST_FILE_EVEN, path, tb);
    assert_eq!(tb.students.len(), 0);
}

#[test]
fn test_get_students_skills_average_green() {
    SETUP_TEAMBUILDER_TEST_AND_INIT!(TEST_FILE_EVEN, path, tb);
    assert!(approx_eq!(
        f32,
        tb.students[0].get_average_skills(),
        2.0,
        F32Margin::default()
    ));
    assert!(approx_eq!(
        f32,
        tb.students[1].get_average_skills(),
        2.166_666_7,
        F32Margin::default()
    ));
}

#[test]
#[should_panic]
fn test_get_students_skills_average_red() {
    SETUP_TEAMBUILDER_TEST_AND_INIT!(TEST_FILE_EVEN, path, tb);
    assert!(approx_eq!(
        f32,
        tb.students[0].get_average_skills(),
        3.0,
        F32Margin::default()
    ));
    assert!(approx_eq!(
        f32,
        tb.students[1].get_average_skills(),
        3.0,
        F32Margin::default()
    ));
}

#[test]
fn test_check_number_of_teams_divisible_first() {
    SETUP_TEAMBUILDER_TEST_AND_INIT!(TEST_FILE_EVEN, path, tb);
    assert_eq!(tb.check_number_of_teams(2), Some((3, 0)));
}

#[test]
fn test_check_number_of_teams_divisible_second() {
    SETUP_TEAMBUILDER_TEST_AND_INIT!(TEST_FILE_EVEN, path, tb);
    assert_eq!(tb.check_number_of_teams(3), Some((2, 0)));
}

#[test]
fn test_check_number_of_teams_not_divisible() {
    SETUP_TEAMBUILDER_TEST_AND_INIT!(TEST_FILE_EVEN, path, tb);
    assert_eq!(tb.check_number_of_teams(4), Some((1, 2)));
}

#[test]
fn test_check_number_of_teams_same_size() {
    SETUP_TEAMBUILDER_TEST_AND_INIT!(TEST_FILE_EVEN, path, tb);
    assert_eq!(tb.check_number_of_teams(6), None);
}

#[test]
fn test_check_number_of_teams_exceed() {
    SETUP_TEAMBUILDER_TEST_AND_INIT!(TEST_FILE_EVEN, path, tb);
    assert_eq!(tb.check_number_of_teams(7), None);
}

#[test]
fn test_calculate_skill_level_green() {
    SETUP_TEAMBUILDER_TEST_AND_INIT!(TEST_FILE_EVEN, path, tb);

    tb.calculate_teams_skill_level();

    let student = tb
        .students
        .iter()
        .find(|&x| x.surname == "Pomettini")
        .unwrap();

    assert!(approx_eq!(
        f32,
        student.average_skill_level,
        1.833_333,
        F32Margin::default()
    ));
}

#[test]
#[should_panic]
fn test_calculate_skill_level_red() {
    SETUP_TEAMBUILDER_TEST_AND_INIT!(TEST_FILE_EVEN, path, tb);

    tb.calculate_teams_skill_level();

    let student = tb
        .students
        .iter()
        .find(|&x| x.surname == "Pomettini")
        .unwrap();

    assert!(approx_eq!(
        f32,
        student.average_skill_level,
        2.0,
        F32Margin::default()
    ));
}

#[test]
fn test_sort_by_skill_level_best_green() {
    SETUP_TEAMBUILDER_TEST_AND_INIT!(TEST_FILE_EVEN, path, tb);

    tb.calculate_teams_skill_level();
    tb.sort_teams_by_skill_level(None);

    assert_eq!(tb.students.last().unwrap().surname, "Bonanni");
}

#[test]
#[should_panic]
fn test_sort_by_skill_level_best_red() {
    SETUP_TEAMBUILDER_TEST_AND_INIT!(TEST_FILE_EVEN, path, tb);

    tb.calculate_teams_skill_level();
    tb.sort_teams_by_skill_level(None);

    assert_eq!(tb.students.last().unwrap().surname, "Pomettini");
}

#[test]
fn test_sort_by_skill_level_worst_green() {
    SETUP_TEAMBUILDER_TEST_AND_INIT!(TEST_FILE_EVEN, path, tb);

    tb.calculate_teams_skill_level();
    tb.sort_teams_by_skill_level(None);

    assert_eq!(tb.students.first().unwrap().surname, "Reclus");
}

#[test]
#[should_panic]
fn test_sort_by_skill_level_worst_red() {
    SETUP_TEAMBUILDER_TEST_AND_INIT!(TEST_FILE_EVEN, path, tb);

    tb.calculate_teams_skill_level();
    tb.sort_teams_by_skill_level(None);

    assert_eq!(tb.students.first().unwrap().surname, "Pomettini");
}

#[test]
fn test_sort_by_skill_best_green() {
    SETUP_TEAMBUILDER_TEST_AND_INIT!(TEST_FILE_EVEN, path, tb);

    tb.calculate_teams_skill_level();
    tb.sort_teams_by_skill_level(Some(SKILL_PROGRAMMING));

    assert_eq!(tb.students.last().unwrap().surname, "Pomettini");
}

#[test]
#[should_panic]
fn test_sort_by_specific_skill_best_red() {
    SETUP_TEAMBUILDER_TEST_AND_INIT!(TEST_FILE_EVEN, path, tb);

    tb.calculate_teams_skill_level();
    // Programming
    tb.sort_teams_by_skill_level(Some(SKILL_PROGRAMMING));

    assert_eq!(tb.students.last().unwrap().surname, "De Dominicis");
}

#[test]
fn test_sort_by_specific_skill_worst_green() {
    SETUP_TEAMBUILDER_TEST_AND_INIT!(TEST_FILE_EVEN, path, tb);

    tb.calculate_teams_skill_level();
    tb.sort_teams_by_skill_level(Some(SKILL_PROGRAMMING));

    assert_eq!(tb.students.first().unwrap().surname, "De Dominicis");
}

#[test]
#[should_panic]
fn test_sort_by_specific_skill_worst_red() {
    SETUP_TEAMBUILDER_TEST_AND_INIT!(TEST_FILE_EVEN, path, tb);

    tb.calculate_teams_skill_level();
    tb.sort_teams_by_skill_level(Some(SKILL_PROGRAMMING));

    assert_eq!(tb.students.first().unwrap().surname, "Pomettini");
}

#[test]
fn test_assign_students_to_team_even() {
    SETUP_TEAMBUILDER_TEST_AND_INIT!(TEST_FILE_EVEN, path, tb);

    tb.calculate_teams_skill_level();
    tb.sort_teams_by_skill_level(None);
    tb.assign_students_to_team(3);

    let first_team: Vec<String> = tb.teams[0]
        .students
        .iter()
        .map(|x| x.surname.clone())
        .collect();

    let second_team: Vec<String> = tb.teams[1]
        .students
        .iter()
        .map(|x| x.surname.clone())
        .collect();

    assert_eq!(tb.teams.len(), 2);
    assert_eq!(first_team, vec!["Bonanni", "Pomettini", "Leotta"]);
    assert_eq!(second_team, vec!["Ricchiuti", "De Dominicis", "Reclus"]);
}

#[test]
fn test_assign_students_to_team_uneven() {
    SETUP_TEAMBUILDER_TEST_AND_INIT!(TEST_FILE_UNEVEN, path, tb);

    tb.calculate_teams_skill_level();
    tb.sort_teams_by_skill_level(None);
    tb.assign_students_to_team(2);

    let first_team: Vec<String> = tb.teams[0]
        .students
        .iter()
        .map(|x| x.surname.clone())
        .collect();

    let second_team: Vec<String> = tb.teams[1]
        .students
        .iter()
        .map(|x| x.surname.clone())
        .collect();

    assert_eq!(tb.teams.len(), 2);
    assert_eq!(first_team, vec!["Bonanni", "Pomettini", "Leotta"]);
    assert_eq!(second_team, vec!["Ricchiuti", "De Dominicis"]);
}
