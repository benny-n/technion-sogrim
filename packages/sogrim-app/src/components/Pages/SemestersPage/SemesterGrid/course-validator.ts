import { courseGradeOptions, emptyRow, RowData } from "../SemesterTabsConsts";

export const validCourseNumber = (
  courseNumber: string,
  semesterRows: RowData[],
  newFlag: boolean
) => {
  let validNumber = /^\d+$/.test(courseNumber) && courseNumber.length === 6;
  if (validNumber && newFlag) {
    const idx = semesterRows.findIndex(
      (row) => row.courseNumber === courseNumber
    );
    if (idx === -1) {
      validNumber = true;
    } else {
      validNumber = false;
    }
  }
  return validNumber;
};

const validCourseCredit = (credit: string | number) => {
  const isNumberOrFloat = Number(credit);
  return (
    isNumberOrFloat === 0 ||
    !!(isNumberOrFloat && +credit >= 0 && (+credit * 2) % 1 === 0)
  );
};

const validGrade = (grade: any) => {
  const gradeNumber = Number(grade);
  if (grade === "" || grade === 0 || grade === "0" || grade === "-") {
    return true;
  }
  if (isNaN(grade)) {
    if (courseGradeOptions.indexOf(grade) > -1) {
      return true;
    } else {
      return false;
    }
  } else if (
    gradeNumber &&
    gradeNumber % 1 === 0 &&
    gradeNumber >= 0 &&
    gradeNumber <= 100
  ) {
    return true;
  }
  return false;
};

export interface courseFromUserValidationsValue {
  error: boolean;
  newRowData: RowData;
  msg: string;
}
export const courseFromUserValidations = (
  course: RowData,
  semesterRows: RowData[],
  newFlag: boolean = false
): courseFromUserValidationsValue => {
  if (!validCourseNumber(course.courseNumber, semesterRows, newFlag)) {
    return {
      error: true,
      newRowData: emptyRow,
      msg: " מספר הקורס שהוזן אינו תקין. מס׳ קורס חייב להכיל 6 ספרות בלבד, וכן אי אפשר לקחת פעמיים קורס באותו הסמסטר.",
    };
  }
  if (!validCourseCredit(course.credit)) {
    return {
      error: true,
      newRowData: emptyRow,
      msg: "נק״ז חייב להיות מספר גדול שווה מאפס, ובקפיצות של 0.5",
    };
  }
  if (!validGrade(course.grade)) {
    return {
      error: true,
      newRowData: emptyRow,
      msg: "ציון חייב להיות מספר בין 0 ל-100 או אחת מהאופציות הלא מספריות (עבר, נכשל, לא השלים)",
    };
  }

  let newState = "לא הושלם";
  let newType =
    course.type === "" || course.type === "-" ? undefined : course.type;
  let newGrade =
    course.grade === "" || course.grade === "-" ? undefined : course.grade;
  if (
    course.grade &&
    (course.grade === "עבר" ||
      course.grade === "פטור ללא ניקוד" ||
      course.grade === "פטור עם ניקוד" ||
      +course.grade >= 55)
  ) {
    newState = "הושלם";
  }
  const fixedNewRowData: RowData = {
    name: course.name,
    courseNumber: course.courseNumber,
    semester: course.semester,
    credit: course.credit,
    state: newState,
    type: newType,
    grade: newGrade,
  };

  return {
    error: false,
    newRowData: fixedNewRowData,
    msg: "",
  };
};
