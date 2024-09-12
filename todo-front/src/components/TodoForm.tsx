import { FC, memo, useState, useCallback, useMemo } from "react";
import { Label, NewTodoPayload } from "../types/todo";
import {
  Box,
  Button,
  TextField,
  Paper,
  Grid,
  Stack,
  Chip,
  Modal,
  FormControlLabel,
  Checkbox,
} from "@mui/material";
import { modalInnerStyle } from "../styles/modal";
import { toggleLabels } from "../lib/toggleLabels";
import styles from "../style.module.css";

type Props = {
  onSubmit: (newTodo: NewTodoPayload) => void;
  labels: Label[];
};

const TodoForm: FC<Props> = memo(({ onSubmit, labels }) => {
  const [editText, setEditText] = useState("");
  const [editLabels, setEditLabels] = useState<Label[]>([]);
  const [openLabelModal, setOpenLabelModal] = useState(false);

  const addTodoHandler = useCallback(() => {
    if (!editText) return;
    onSubmit({
      text: editText,
      labels: editLabels.map((label) => label.id),
    });
    setEditText("");
    setEditLabels([]);
  }, [editText, editLabels, onSubmit]);

  const handleOpenModal = useCallback(() => setOpenLabelModal(true), []);
  const handleCloseModal = useCallback(() => setOpenLabelModal(false), []);

  const labelChips = useMemo(
    () => editLabels.map((label) => <Chip key={label.id} label={label.name} />),
    [editLabels]
  );

  const labelCheckboxes = useMemo(
    () =>
      labels.map((label) => (
        <FormControlLabel
          key={label.id}
          control={<Checkbox checked={editLabels.includes(label)} />}
          label={label.name}
          onChange={() => setEditLabels((prev) => toggleLabels(prev, label))}
        />
      )),
    [labels, editLabels]
  );

  return (
    <Paper elevation={2}>
      <Box className={styles.form}>
        <Grid
          container
          rowSpacing={2}
          columnSpacing={5}
          className={styles.form_area}
        >
          <Grid item xs={12}>
            <TextField
              label="New Todo"
              variant="filled"
              value={editText}
              onChange={(e) => setEditText(e.target.value)}
              className={styles.form_text}
            />
          </Grid>
          <Grid item xs={12}>
            <Stack direction="row" spacing={1}>
              {labelChips}
            </Stack>
          </Grid>
          <Grid item xs={2}>
            <Button
              onClick={handleOpenModal}
              color="secondary"
              className={styles.form_button}
            >
              Select Labels
            </Button>
          </Grid>
          <Grid item xs={1.5}>
            <Button onClick={addTodoHandler} className={styles.form_button}>
              Add Todo
            </Button>
          </Grid>
        </Grid>
        <Modal open={openLabelModal} onClose={handleCloseModal}>
          <Box sx={modalInnerStyle}>
            <Stack>{labelCheckboxes}</Stack>
          </Box>
        </Modal>
      </Box>
    </Paper>
  );
});

export default TodoForm;
