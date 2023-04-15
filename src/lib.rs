/// *  If the difference between the grade and the next multiple of 5
///  is less than 3, round grade up to the next multiple of 5.
///  *  If the value of grade is less than 38, no rounding occurs
///   as the result will still be a failing grade.
fn gradingStudents(grades: &[i32]) -> Vec<i32> {
    grades
        .iter()
        .map(|&grade| {
            if grade < 38 {
                grade
            } else {
                let next_multiple_of_5 = (grade / 5 + 1) * 5;
                if next_multiple_of_5 - grade < 3 {
                    next_multiple_of_5
                } else {
                    grade
                }
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grading_students() {
        let grades = [73, 67, 38, 33];
        let expected = [75, 67, 40, 33];
        assert_eq!(gradingStudents(&grades), expected);
    }
}
