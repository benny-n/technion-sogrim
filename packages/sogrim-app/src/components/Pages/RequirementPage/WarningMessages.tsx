import { Box, Divider, Typography, useTheme } from "@mui/material";
import { observer } from "mobx-react-lite";
import React from "react";
import useUserState from "../../../hooks/apiHooks/useUserState";
import { MessagesAccordion } from "./MessagesAccordion";

interface WarningMessagesProps {}

const error = "פסילה: ";
const warning = "אזהרה: ";

const WarningMessagesComp: React.FC<WarningMessagesProps> = () => {
  const { data: userState } = useUserState();
  const theme = useTheme();

  const messages: string[] = React.useMemo(
    () =>
      userState?.details?.degree_status?.overflow_msgs.filter(
        (ovm) => ovm.match(error) || ovm.match(warning)
      ) || [],
    [userState]
  );

  return messages.length > 0 ? (
    <MessagesAccordion
      name={`אזהרות - ${userState?.details.catalog?.name.replace(
        /\d{4}-\d{4}/, // remove the year from the catalog name
        ""
      )}`}
      tooltipMsg="כאן מופיעות הודעות אזהרה על אי עמידה בתנאי הקטלוג האקדמי"
      Messages={() => (
        <>
          {messages.map((ovm, id) => (
            <Box key={id} sx={{ padding: 0.5 }}>
              {ovm.match(error) ? (
                <Typography color="error">{ovm.replace(error, "")}</Typography>
              ) : (
                <Typography color={theme.palette.warning.main}>
                  {ovm.replace(warning, "")}
                </Typography>
              )}
              <Divider />
            </Box>
          ))}
        </>
      )}
    />
  ) : null;
};

export const WarningMessages = observer(WarningMessagesComp);
