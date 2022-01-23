import {
  Box,
  TextField,
  Button,
  Card,
  CardContent,
  Typography,
} from "@mui/material";
import { observer } from "mobx-react-lite";
import React, { useEffect, useState } from "react";
import useUpdateCourse from "../../../../hooks/apiHooks/useUpdateCourse";
import { useStore } from "../../../../hooks/useStore";
import { useAuth } from "../../../../hooks/useAuth";
import { Course } from "../../../../types/data-types";

interface CoursePageUpdateProps {}

const emptyCourse = {
  name: "",
  credit: 0,
  _id: "",
};
const CoursePageUpdateComp: React.FC<CoursePageUpdateProps> = () => {
  const {
    dataStore: { addCourse, updateCourse },
    uiStore: { currentSelectedCourse },
  } = useStore();

  const { userAuthToken } = useAuth();
  const { mutate } = useUpdateCourse(userAuthToken);

  const [updatedCourse, setUpdatedCourse] = useState<Course | undefined>(
    currentSelectedCourse
  );

  const [mode, setMode] = useState<"update" | "add">("add");

  useEffect(() => {
    setUpdatedCourse(currentSelectedCourse);
  }, [currentSelectedCourse]);

  const handleEditChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    let fieldName, fieldValue;
    event.preventDefault();
    fieldName = event.target?.getAttribute("name") as keyof Course;
    fieldValue = event.target.value;
    if (updatedCourse) {
      let newUpdatedCourse: Course = { ...updatedCourse };
      if (fieldName === "credit") {
        fieldValue = +fieldValue;
      }
      // @ts-ignore
      newUpdatedCourse[fieldName] = fieldValue;
      setUpdatedCourse(newUpdatedCourse);
    }
  };

  const handleUpdateClick = () => {
    if (updatedCourse) {
      if (updateCourse(updatedCourse)) {
        mutate(updatedCourse);
      }
    }
  };

  const handleAddClick = () => {
    if (updatedCourse) {
      if (addCourse(updatedCourse)) {
        mutate(updatedCourse);
      }
    }
  };

  const changeMode = () => {
    if (mode === "add") {
      setMode("update");
    } else {
      setUpdatedCourse(emptyCourse);
      setMode("add");
    }
  };

  const buttonModeTitle = mode === "add" ? "עדכן קורס" : "הוסף קורס";
  const buttonTitle = mode === "add" ? "הוסף קורס" : "עדכן קורס";
  const displayCourse = mode === "add" ? emptyCourse : updatedCourse;

  return (
    <Box sx={{ display: "flex", flexDirection: "column", gap: 2 }}>
      <TextField
        name="name"
        sx={{ minWidth: 400 }}
        required
        id="outlined-name"
        label="שם הקורס"
        value={displayCourse?.name}
        onChange={handleEditChange}
      />
      <TextField
        disabled={mode === "update"}
        required
        name="_id"
        id="outlined-id"
        label="מספר הקורס"
        value={displayCourse?._id}
        onChange={handleEditChange}
      />
      <TextField
        type="number"
        required
        name="credit"
        id="outlined-credit"
        label="נק״ז"
        value={displayCourse?.credit}
        onChange={handleEditChange}
      />
      <Box sx={{ display: "flex", justifyContent: "space-between" }}>
        <Button
          sx={{ minWidth: 200 }}
          size="large"
          onClick={mode === "add" ? handleAddClick : handleUpdateClick}
          variant="contained"
          color="info"
        >
          {buttonTitle}
        </Button>
        <Button
          sx={{ minWidth: 150 }}
          size="large"
          onClick={changeMode}
          variant="outlined"
          color="info"
        >
          החלף ל{buttonModeTitle}
        </Button>
      </Box>
      <Card>
        <CardContent>
          <Typography variant="body2" color="text.secondary">
            מידע על עדכן - חדש וכו׳
          </Typography>
        </CardContent>{" "}
      </Card>
    </Box>
  );
};

export const CoursePageUpdate = observer(CoursePageUpdateComp);
