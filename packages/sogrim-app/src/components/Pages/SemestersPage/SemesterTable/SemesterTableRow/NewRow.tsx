import { useState, useMemo } from "react";
import { useStore } from '../../../../../hooks/useStore';
import { Grid, IconButton, MenuItem, TableCell, TextField, Tooltip, Select  } from "@mui/material";
import AutoFixNormalOutlinedIcon from '@mui/icons-material/AutoFixNormalOutlined';
import { RowData } from "../SemesterTabsConsts";
import { courseGradeOptions, courseStateOptions } from '../SemesterTabsConsts'
import { NewRowActionCell } from "./NewRowActionCell";

export interface NewRowProps {
    newRow: RowData;
    handleEditChange: any;
    handleAdd: any;
    handleCancel: any;
    labelId: string;
}

const NewRowComp: React.FC<NewRowProps> = ({
    newRow,
    handleEditChange,
    handleCancel,
    handleAdd,
    labelId,
}) => {      

  const { dataStore: {
    getUserBankNames,
  } } = useStore();

  const { name, courseNumber, credit, grade, type, state} = newRow;

  const banksNamesOptions = useMemo(() => getUserBankNames(), []);


  const [ gradeToggle, setGradeToggle] = useState<boolean>(true);

  const gradeToggleClick = () => setGradeToggle(!gradeToggle);

const gradeNonNumericDefualt = (gradeToTranslate: string) => {
    const idx = courseGradeOptions.findIndex(gradeOpt => gradeOpt === gradeToTranslate);
    return idx === -1 ? courseGradeOptions[0] : courseGradeOptions[idx];
  }

  return (
      <>
          <TableCell
              align='center'
              component="th"
              id={labelId}
              scope="row"
              padding="none"
              width={'250px'}
          >
              <TextField id="course-name" name="name" onChange={handleEditChange}                       
                         value={name} variant="outlined" size="small"/>                
          </TableCell>
          <TableCell align="center" width={'150px'}>
              <TextField id="course-number" name="courseNumber" onChange={handleEditChange}
                         value={courseNumber} variant="outlined" size="small"/>                 
          </TableCell>
          <TableCell align="center" width={'80px'}>
              <TextField id="course-credit" name="credit" onChange={handleEditChange}                          
                          value={credit} variant="outlined" size="small" type="tel"/>  
          </TableCell>
            <TableCell align="center" width={'250px'} >
              <Grid container justifyContent={'center'} direction={'row'}>
                  <Tooltip title={gradeToggle ? 'ציון לא מספרי' : 'ציון מספרי'} arrow> 
                  <IconButton color="primary" onClick={gradeToggleClick}>
                    <AutoFixNormalOutlinedIcon />
                  </IconButton>
                  </Tooltip>
                { gradeToggle ?
                  <TextField id="course-grade" name="grade" onChange={handleEditChange}                       
                           value={grade} variant="outlined" size="small" type="number"/> 
                  : 
                  <Select   id="course-grade" value={gradeNonNumericDefualt(grade)} name="grade" 
                            onChange={(event, newValue) => handleEditChange(event, 'grade', newValue)}
                            variant="outlined" size="small" sx={{width: '170px'}}> 
              {   courseGradeOptions.map((option) => (
                  <MenuItem key={option} value={option}>
                  {option}
                  </MenuItem>
        ))}
              </Select>
                    }
              </Grid>
          </TableCell>
            <TableCell align="center" width={'170px'}>
              <Select id="course-type" name="type" onChange={(event, newValue) => handleEditChange(event, 'type', newValue)}                 
                        value={newRow.type} variant="outlined" size="small" fullWidth> 
                {banksNamesOptions.map((option) => (
                          <MenuItem key={option} value={option}>
                          {option}
                          </MenuItem>
                ))}
              </Select>
          </TableCell>
          <TableCell align="center" width={'170x'}>
              <Select id="course-state" name="state" onChange={(event, newValue) => handleEditChange(event, 'state', newValue)}
              value={newRow.state} variant="outlined" size="small" fullWidth> 
              {courseStateOptions.map((option) => (
                  <MenuItem key={option} value={option}>
                  {option}
                  </MenuItem>
        ))}
              </Select>
          </TableCell>    
          <NewRowActionCell handleAdd={handleAdd} handleCancel={handleCancel}/>      
      </>
  )
}

export const NewRow = NewRowComp;