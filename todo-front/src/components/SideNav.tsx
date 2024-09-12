import {
  Button,
  IconButton,
  List,
  ListItem,
  ListItemButton,
  Modal,
  Stack,
  TextField,
  Typography,
} from "@mui/material";
import { Box } from "@mui/system";
import LabelIcon from "@mui/icons-material/Label";
import EditIcon from "@mui/icons-material/Edit";
import DeleteIcon from "@mui/icons-material/Delete";
import { useState, FC, memo, useCallback, useMemo, Fragment } from "react";
import { modalInnerStyle } from "../styles/modal";
import { Label, NewLabelPayload } from "../types/todo";
import styles from "../style.module.css";

type Props = {
  labels: Label[];
  filterLabelId: number | null;
  onSelectLabel: (label: Label | null) => void;
  onSubmitNewLabel: (newLabel: NewLabelPayload) => void;
  onDeleteLabel: (id: number, name: string) => void;
  deleteError: string | null;
  onResetErrText: () => void;
};

const selected = "#1976d2";

const SideNav: FC<Props> = memo(
  ({
    labels,
    filterLabelId,
    onSelectLabel,
    onSubmitNewLabel,
    onDeleteLabel,
    deleteError,
    onResetErrText,
  }) => {
    const [editName, setEditName] = useState("");
    const [openLabelModal, setOpenLabelModal] = useState(false);

    const handleOpenModal = useCallback(() => {
      setOpenLabelModal(true);
      onResetErrText();
    }, [onResetErrText]);

    const handleCloseModal = useCallback(() => {
      setOpenLabelModal(false);
      onResetErrText();
    }, [onResetErrText]);

    const onSubmit = useCallback(() => {
      if (editName.trim() === "") return;
      onSubmitNewLabel({ name: editName });
      setEditName("");
    }, [editName, onSubmitNewLabel]);

    const labelList = useMemo(
      () =>
        labels.map((label) => (
          <ListItem key={label.id} disablePadding>
            <ListItemButton
              onClick={() =>
                onSelectLabel(label.id === filterLabelId ? null : label)
              }
              selected={label.id === filterLabelId}
              sx={{
                bgcolor: label.id === filterLabelId ? selected : "transparent",
                color: label.id === filterLabelId ? selected : "inherit",
              }}
            >
              <Stack direction="row" alignItems="center" spacing={1}>
                <LabelIcon fontSize="small" />
                <span>{label.name}</span>
              </Stack>
            </ListItemButton>
          </ListItem>
        )),
      [labels, filterLabelId, onSelectLabel]
    );

    // 改行ありのテキストを出力
    const MultiLineBody = ({ body }: { body: string | null }) => {
      const texts = body?.split("\n").map((item, index) => {
        return (
          <Fragment key={index}>
            {item}
            <br />
          </Fragment>
        );
      });
      return <div>{texts}</div>;
    };

    return (
      <>
        <List>
          {labelList}
          <ListItem disablePadding>
            <ListItemButton onClick={handleOpenModal}>
              <Stack direction="row" alignItems="center" spacing={1}>
                <EditIcon fontSize="small" />
                <span>edit label</span>
              </Stack>
            </ListItemButton>
          </ListItem>
        </List>
        <Modal open={openLabelModal} onClose={handleCloseModal}>
          <Box sx={modalInnerStyle}>
            <Stack spacing={3}>
              <Stack spacing={1}>
                <Typography variant="subtitle1">New Label</Typography>
                <TextField
                  label="new label"
                  variant="filled"
                  fullWidth
                  value={editName}
                  onChange={(e) => setEditName(e.target.value)}
                />
                <Box textAlign="right">
                  <Button onClick={onSubmit}>Submit</Button>
                </Box>
              </Stack>
              <div className={styles.error_text}>
                <MultiLineBody body={deleteError} />
              </div>
              <Stack spacing={1}>
                {labels.map((label) => (
                  <Stack
                    key={label.id}
                    direction="row"
                    alignItems="center"
                    spacing={1}
                  >
                    <IconButton
                      size="small"
                      onClick={() => {
                        onDeleteLabel(label.id, label.name);
                      }}
                    >
                      <DeleteIcon fontSize="small" />
                    </IconButton>
                    <span>{label.name}</span>
                  </Stack>
                ))}
              </Stack>
            </Stack>
          </Box>
        </Modal>
      </>
    );
  }
);

export default SideNav;
