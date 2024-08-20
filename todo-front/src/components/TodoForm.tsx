import { FC, useState } from "react";
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

const TodoForm: FC<Props> = ({ onSubmit, labels }) => {
  const [editText, setEditText] = useState("");
  const [editLabels, setEditLabels] = useState<Label[]>([]);
  const [openLabelModal, setOpenLabelModal] = useState(false);

  const addTodoHandler = async () => {
    if (!editText) return;

    onSubmit({
      text: editText,
      labels: editLabels.map((label) => label.id),
    });
    setEditText("");
  };

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
              label="new todo text"
              variant="filled"
              value={editText}
              onChange={(e) => setEditText(e.target.value)}
              fullWidth
            />
          </Grid>
          <Grid item xs={12}>
            <Stack direction="row" spacing={1}>
              {editLabels.map((label) => (
                <Chip key={label.id} label={label.name} />
              ))}
            </Stack>
          </Grid>
          <Grid item xs={3} xl={7}>
            <Button
              onClick={() => setOpenLabelModal(true)}
              fullWidth
              color="secondary"
              sx={{
                maxWidth: 128,
              }}
            >
              select label
            </Button>
          </Grid>
          <Grid item xs={3}>
            <Button
              onClick={addTodoHandler}
              fullWidth
              sx={{
                maxWidth: 128,
              }}
            >
              add todo
            </Button>
          </Grid>
          <Modal open={openLabelModal} onClose={() => setOpenLabelModal(false)}>
            <Box sx={modalInnerStyle}>
              <Stack>
                {labels.map((label) => (
                  <FormControlLabel
                    key={label.id}
                    control={<Checkbox checked={editLabels.includes(label)} />}
                    label={label.name}
                    onChange={() =>
                      setEditLabels((prev) => toggleLabels(prev, label))
                    }
                  />
                ))}
              </Stack>
            </Box>
          </Modal>
        </Grid>
      </Box>
    </Paper>
  );
};

export default TodoForm;
