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
      <Box sx={{ p: 2 }}>
        <Grid
          container
          rowSpacing={2}
          columnSpacing={5}
          sx={{ justifyContent: "space-between" }}
        >
          <Grid item xs={12}>
            <TextField
              label="New Todo"
              variant="filled"
              value={editText}
              onChange={(e) => setEditText(e.target.value)}
              fullWidth
            />
          </Grid>
          <Grid item xs={12}>
            <Stack direction="row" spacing={1}>
              {labelChips}
            </Stack>
          </Grid>
          <Grid item xs={3} xl={7}>
            <Button
              onClick={handleOpenModal}
              fullWidth
              color="secondary"
              sx={{ maxWidth: 128 }}
            >
              Select Labels
            </Button>
          </Grid>
          <Grid item xs={3}>
            <Button onClick={addTodoHandler} fullWidth>
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
