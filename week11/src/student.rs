// The `dead_code` allow covers types and methods that are defined here but only
// exercised by the test suite. Once you implement everything and uncomment the
// demo code in main.rs, these warnings disappear naturally.
#![allow(dead_code)]

use std::collections::HashMap;

// ============================================================================
// TYPES — do not modify these definitions
// ============================================================================

pub struct Student {
    pub id: String,
    pub name: String,
    pub email: String,
    pub credits_earned: u16,
    pub grades: Vec<CourseGrade>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Grade {
    A,
    B,
    C,
    D,
    F,
}

#[derive(Debug, Clone)]
pub struct CourseGrade {
    pub course_code: String,
    pub course_name: String,
    pub credits: u16,
    pub grade: Grade,
}

pub struct StudentDatabase {
    students: HashMap<String, Student>,
}

// ============================================================================
// IMPLEMENTATIONS — replace every todo!() with a real implementation.
// When you do, remove the leading `_` from each parameter name.
// ============================================================================

impl Student {
    /// Creates a new student with the given id, name, and email.
    /// `credits_earned` starts at 0 and `grades` starts empty.
    pub fn new(id: String, name: String, email: String) -> Student {
        Student {
            id,
            name,
            email,
            credits_earned: 0,
            grades: Vec::new(),
        }
    }

    /// Returns a string describing the student's class standing based on credits:
    ///   0–29   → "Freshman"
    ///   30–59  → "Sophomore"
    ///   60–89  → "Junior"
    ///   90+    → "Senior"
    pub fn class_standing(&self) -> &str {
        if self.credits_earned >= 90 {
            "Senior"
        } else if self.credits_earned >= 60 {
            "Junior"
        } else if self.credits_earned >= 30 {
            "Sophomore"
        } else {
            "Freshman"
        }
    }

    /// Adds `credits` to the student's `credits_earned` total.
    pub fn add_credits(&mut self, credits: u16) {
        self.credits_earned += credits;
    }

    /// Returns `true` if the student has earned 120 or more credits.
    pub fn can_graduate(&self) -> bool {
        self.credits_earned >= 120
    }

    /// Appends `course_grade` to the student's `grades` vector.
    pub fn add_grade(&mut self, course_grade: CourseGrade) {
        self.grades.push(course_grade);
    }

    /// Returns the student's GPA as a weighted average using quality points.
    /// Returns 0.0 if the student has no grades.
    ///
    /// GPA = total quality points / total credit hours
    pub fn calculate_gpa(&self) -> f32 {
        if self.grades.is_empty() {
            return 0.0;
        }

        let total_quality_points: f32 = self.grades.iter().map(|g| g.quality_points()).sum();
        let total_credit_hours: u16 = self.grades.iter().map(|g| g.credits).sum();

        if total_credit_hours == 0 {
            0.0
        } else {
            total_quality_points / total_credit_hours as f32
        }
    }
}

impl Grade {
    /// Returns the GPA points for this letter grade:
    ///   A → 4.0, B → 3.0, C → 2.0, D → 1.0, F → 0.0
    pub fn to_gpa_points(&self) -> f32 {
        match self {
            Grade::A => 4.0,
            Grade::B => 3.0,
            Grade::C => 2.0,
            Grade::D => 1.0,
            Grade::F => 0.0,
        }
    }

    /// Parses a grade from a string (case-insensitive).
    /// Returns `None` for unrecognised inputs.
    ///
    /// # Examples
    /// ```
    /// assert_eq!(Grade::from_string("A"), Some(Grade::A));
    /// assert_eq!(Grade::from_string("a"), Some(Grade::A));
    /// assert_eq!(Grade::from_string("Z"), None);
    /// ```
    pub fn from_string(s: &str) -> Option<Grade> {
        match s.to_uppercase().as_str() {
            "A" => Some(Grade::A),
            "B" => Some(Grade::B),
            "C" => Some(Grade::C),
            "D" => Some(Grade::D),
            "F" => Some(Grade::F),
            _ => None,
        }
    }

    /// Returns `true` for grades A, B, and C; `false` for D and F.
    pub fn is_passing(&self) -> bool {
        matches!(self, Grade::A | Grade::B | Grade::C)
    }
}

impl CourseGrade {
    /// Creates a new CourseGrade.
    pub fn new(
        course_code: String,
        course_name: String,
        credits: u16,
        grade: Grade,
    ) -> CourseGrade {
        CourseGrade {
            course_code,
            course_name,
            credits,
            grade,
        }
    }

    /// Returns the quality points for this course: credits × GPA points.
    pub fn quality_points(&self) -> f32 {
        self.credits as f32 * self.grade.to_gpa_points()
    }
}

impl StudentDatabase {
    /// Creates a new, empty database.
    pub fn new() -> StudentDatabase {
        StudentDatabase {
            students: HashMap::new(),
        }
    }

    /// Adds a student to the database.
    /// Returns `Err` if a student with the same id already exists.
    pub fn add_student(&mut self, student: Student) -> Result<(), String> {
        if self.students.contains_key(&student.id) {
            Err(format!("Student with id {} already exists", student.id))
        } else {
            self.students.insert(student.id.clone(), student);
            Ok(())
        }
    }

    /// Returns a reference to the student with the given id, or `None`.
    pub fn find_student(&self, id: &str) -> Option<&Student> {
        self.students.get(id)
    }

    /// Returns a mutable reference to the student with the given id, or `None`.
    pub fn find_student_mut(&mut self, id: &str) -> Option<&mut Student> {
        self.students.get_mut(id)
    }

    /// Returns the total number of students in the database.
    pub fn student_count(&self) -> usize {
        self.students.len()
    }

    /// Returns the average GPA across all students.
    /// Returns 0.0 if there are no students.
    pub fn average_gpa(&self) -> f32 {
        if self.students.is_empty() {
            return 0.0;
        }

        let total_gpa: f32 = self
            .students
            .values()
            .map(|student| student.calculate_gpa())
            .sum();
        total_gpa / self.students.len() as f32
    }

    /// Returns a vector of references to all students in the database.
    pub fn list_students(&self) -> Vec<&Student> {
        self.students.values().collect()
    }
}
