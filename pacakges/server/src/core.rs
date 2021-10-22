use rocket::http::Status;
use rocket_db_pools::Connection;
use crate::db::{Db, services::get_catalog};
use bson::{Bson, doc};

//

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct UserDetails {
    pub courses: Vec<CourseStatus>, //from parser
    pub catalog : bson::oid::ObjectId,
    pub degree_status: DegreeStatus,
}

impl UserDetails {
    pub fn find_course_by_number(&self, number: u32) -> Option<CourseStatus>{
        for course in &self.courses {
            if course.course.number == number {
                return Some(course.clone());
            }
        }
        None
    }
}
    
#[derive(Default, Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct User {
    #[serde(rename(serialize = "_id", deserialize = "id"))]
    pub id : bson::oid::ObjectId,
    pub name: String,
    pub details : Option<UserDetails>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum Rule {
    All, //  כמו חובה פקולטית.
    Accumulate(u8), // לצבור איקס נקודות מתוך הבנק. למשל, רשימה א'
    Chain(Vec<Vec<Course>>), // למשל שרשרת מדעית.
    SpecializationGroups(Vec<SpecializationGroup>),
    Wildcard(bool), // קלף משוגע עבור להתמודד עם   
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Course {
    pub number : u32,
    pub credit: f32,
    pub name: String,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum CourseState{
    Complete,
    NotComplete,
    InProgress,
}

impl CourseStatus {
    pub fn passed(&self) -> bool {
        match &self.grade {
            Some(grade) => {
                match grade{
                    Grade::Grade(grade) => grade >= &55,
                    Grade::Binary(val) => *val,
                    Grade::ExemptionWithoutCredit => true,
                    Grade::ExemptionWithCredit => true,
                } 
            },
            None => false,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum Grade{
    Grade(u8),
    Binary(bool),
    ExemptionWithoutCredit,
    ExemptionWithCredit,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct CourseStatus{
    pub course: Course,
    pub state: Option<CourseState>,
    pub semester : Option<String>,
    pub grade : Option<Grade>,
    pub r#type : Option<String>, // if none, nissan cries 
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Requirement{
    /*
    בזין הזה יש את כל הבנקים והאם בוצעו או לא בכל קטלוג
    */
    pub course_bank_name: String,
    pub credit_requirment: f32,
    pub credit_complete: f32,
    // TODO planing ...
    pub message: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct DegreeStatus {
    pub course_statuses: Vec<CourseStatus>,
    pub course_bank_requirements: Vec<Requirement>, // 
    pub credit_overflow: Option<Vec<String>>, // זליגות של נקז ואיך טיפלנו בהם
    pub total_credit: f32,   
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Catalog{
    #[serde(rename(serialize = "_id", deserialize = "id"))]
    pub id : bson::oid::ObjectId,
    pub name: String,
    pub course_banks: Vec<CourseBank>,
    pub course_table: Vec<CourseTableRow>
}

impl Catalog {
    fn get_course_list_for_bank(&self, bank_name: &String) -> Vec<u32> {
        let mut course_list_for_bank = Vec::<u32>::new();
        for course in &self.course_table {
            if course.course_banks.contains(bank_name) {
                course_list_for_bank.push(course.number);
            }
        }
        course_list_for_bank
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct CourseBank{
    pub name: String, // for example, Hova, Rshima A.
    pub rule: Rule,
    pub credit: f32,
    pub messege: String, //
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct SpecializationGroup {
    pub name: String,
    pub credit: f32,
    pub mandatory: Option<(Vec<u32>, Logic)>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum Logic {
    OR,
    AND,
}

#[derive(Default, Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct CourseTableRow {
    pub number: u32,
    pub course_banks: Vec<String> // שמות הבנקים. שימו לב לקבוצת ההתמחות
}

pub fn set_order(course_banks_type: &Vec::<CourseBank>) -> &Vec::<CourseBank> {
    course_banks_type
}

// dummy function, need to be implmeneted by Benny
pub async fn get_course(course_id : u32, conn: &Connection<Db>) -> Result<Course, Status> {
    Ok(Course {
        number: 111111,
        credit: 0.0,
        name: String::from("temp"),
    })
}

pub async fn handle_bank_rule_all(bank_name: &String, degree_status: &mut DegreeStatus,
                                  course_list: &Vec<u32>, user: &UserDetails, conn: &Connection<Db>) -> Result<f32,Status> {
    let mut sum_credits = 0.0;
    for course_number in course_list {
        match user.find_course_by_number(course_number.clone()) {
            Some(course_status) => {
                degree_status.course_statuses.push(CourseStatus {
                    course: course_status.course.clone(),
                    r#type : Some(bank_name.clone()),
                    state: if course_status.passed() { Some(CourseState::Complete) } else { Some(CourseState::NotComplete) },
                    semester: course_status.semester.clone(),
                    grade: course_status.grade.clone(),
                });
                if course_status.passed() {
                    sum_credits += course_status.course.credit;
                }
            },
            None => {
                degree_status.course_statuses.push(CourseStatus {
                    course : get_course(course_number.clone(), conn).await?,
                    r#type : Some(bank_name.clone()),
                    state : Some(CourseState::NotComplete),
                    semester : None,
                    grade : None,
                });
            },
        }
    }
    Ok(sum_credits)
}

pub async fn calculate_degree_status(user: &UserDetails, conn: &Connection<Db>) -> Result<(),Status> {
    let catalog = get_catalog(user.catalog, conn).await?;
    let course_banks = set_order(&catalog.course_banks);
    let mut degree_status = DegreeStatus {
        course_statuses: Vec::<CourseStatus>::new(),
        course_bank_requirements: Vec::<Requirement>::new(),
        credit_overflow: None,
        total_credit: 0.0,
    };
    for bank in course_banks {
        let course_list_for_bank = catalog.get_course_list_for_bank(&bank.name);
        match &bank.rule {
            Rule::All => { 
                let sum_credits = handle_bank_rule_all(&bank.name, &mut degree_status, &course_list_for_bank, &user, &conn).await?;
                degree_status.course_bank_requirements.push(Requirement {
                    course_bank_name: bank.name.clone(),
                    credit_requirment: bank.credit,
                    credit_complete: sum_credits,
                    message: None,
                });

            }
            _ => todo!()
        }
    }
    Ok(())
    
}