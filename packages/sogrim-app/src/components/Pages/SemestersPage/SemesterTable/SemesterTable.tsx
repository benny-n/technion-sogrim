import * as React from 'react';
import Box from '@mui/material/Box';
import Table from '@mui/material/Table';
import TableContainer from '@mui/material/TableContainer';
import { observer } from 'mobx-react-lite';
import { RowData } from './SemesterTabsConsts';
import { Paper } from '@mui/material';
import { SemesterTableHeader } from './SemesterTableHeader';
import { useStore } from '../../../../hooks/useStore';
import { useAuth } from '../../../../hooks/useAuth';
import useUserState from '../../../../hooks/apiHooks/useUserState';
import useComputeEndGame from '../../../../hooks/apiHooks/useComputeEndGame';
import useUpdateUserState from '../../../../hooks/apiHooks/useUpdateUserState';
import { SemesterTableBody } from './SemesterTableBody';
export interface SemesterTableProps {    
    semester: string;    
  }

const SemesterTableComp: React.FC<SemesterTableProps> = ({    
    semester,
}) => {

    const { dataStore: {            
            generateRows,
            updateCourseInUserDetails,            
          }
  } = useStore();

  const { userAuthToken } = useAuth();
  const { data, isLoading, refetch } = useUserState(userAuthToken);
  const { mutate } = useUpdateUserState(userAuthToken);

  const [tableRows, setTableRows] = React.useState<RowData[]>([]);

  React.useEffect(() => {
    if (data) {
      setTableRows(generateRows(semester, data?.details.degree_status.course_statuses));      
    }
  },[ data, isLoading, generateRows, semester ]);

  const handleSave = (newRowData: RowData, semester: string) => {
    if (!isLoading && data && data?.details) {
      const newUserDetails = updateCourseInUserDetails(newRowData, semester, data?.details);      
      mutate(newUserDetails);
      console.log('hi hi hi' , data?.details.degree_status.course_statuses)
      const newnewrow = generateRows(semester, data?.details.degree_status.course_statuses);
      console.log('~~~~~~ NEW! ', newnewrow);
      setTableRows(newnewrow);      
    }
  }
    
  return (
    <Box sx={{ width: '100%', display: 'flex', alignItems: 'center'}}>
      <Paper sx={{ width: '100%', mb: 2 }}>        
        <TableContainer sx={{ width: '1200px' }}>
          <Table            
            aria-labelledby="tableTitle"
            size={'small'}
          >
            <SemesterTableHeader                
            />
            <SemesterTableBody tableRows={tableRows} semester={semester} handleSave={handleSave}/>
          </Table>
        </TableContainer>    
        {/* <TableFooter>
            Add a new row
        </TableFooter>     */}
      </Paper>      
    </Box>
  );
}

export const SemesterTable = observer(SemesterTableComp);