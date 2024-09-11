import {
  ChangeEventHandler,
  FC,
  memo,
  useEffect,
  useState,
  // useCallback,
} from "react";
import { Todo, UpdateTodoPayload, Label } from "../types/todo";
import {
  Button,
  Card,
  Checkbox,
  Grid,
  Stack,
  Typography,
  Modal,
  Box,
  TextField,
  Chip,
  FormControlLabel,
  CircularProgress,
} from "@mui/material";
import { modalInnerStyle } from "../styles/modal";
import { toggleLabels } from "../lib/toggleLabels";

type Props = {
  todo: Todo;
  onUpdate: (todo: UpdateTodoPayload) => Promise<void>; // Promiseを返す非同期関数に変更
  onDelete: (id: number) => Promise<void>; // Promiseを返す非同期関数に変更
  labels: Label[];
};

const TodoItem: FC<Props> = memo(({ todo, onUpdate, onDelete, labels }) => {
  const [editing, setEditing] = useState(false);
  const [editText, setEditText] = useState("");
  const [editLabels, setEditLabels] = useState<Label[]>([]);
  const [isLoading, setIsLoading] = useState(false); // ローディング状態を追加

  useEffect(() => {
    setEditText(todo.text);
    setEditLabels(todo.labels);
  }, [todo, editing]);

  const handleCompletedCheckbox: ChangeEventHandler = () => {
    setIsLoading(true); // ローディング開始
    onUpdate({
      ...todo,
      completed: !todo.completed,
      labels: todo.labels.map((label) => label.id),
    }).finally(() => setIsLoading(false)); // ローディング終了
  };

  const onCloseEditModal = () => {
    setIsLoading(true); // ローディング開始
    onUpdate({
      ...todo,
      text: editText,
      completed: todo.completed,
      labels: editLabels.map((label) => label.id),
    }).finally(() => setIsLoading(false)); // ローディング終了
    setEditing(false);
  };

  const handleDelete = () => {
    setIsLoading(true); // ローディング開始
    onDelete(todo.id).finally(() => setIsLoading(false)); // ローディング終了
  };

  return (
    <Card sx={{ p: 1 }}>
      <Grid container spacing={2} alignItems="center">
        <Grid item xs={1}>
          <Checkbox
            onChange={handleCompletedCheckbox}
            checked={todo.completed}
          />
        </Grid>

        <Grid item xs={8} sx={{ flexBasis: 100, ml: 2 }}>
          <Stack spacing={1}>
            <Typography
              variant="caption"
              fontSize={16}
              sx={{ paddingTop: "6px" }}
            >
              {todo.text}
            </Typography>
            <Stack direction="row" spacing={1}>
              {todo.labels?.map((label) => (
                <Chip key={label.id} label={label.name} />
              ))}
            </Stack>
          </Stack>
        </Grid>
        <Grid item xs={2}>
          <Stack direction="row" spacing={1}>
            <Button onClick={() => setEditing(true)} color="info">
              edit
            </Button>
            <Button onClick={handleDelete} color="error">
              delete
            </Button>
          </Stack>
        </Grid>
      </Grid>

      {/* ローディング中にスピナーを表示 */}
      {isLoading && <CircularProgress size={24} sx={{ ml: 10 }} />}

      <Modal open={editing} onClose={onCloseEditModal}>
        <Box sx={modalInnerStyle}>
          <Stack spacing={2}>
            <TextField
              size="small"
              label="todo text"
              defaultValue={todo.text}
              onChange={(e) => setEditText(e.target.value)}
            />
          </Stack>
          <Stack>
            <Typography variant="subtitle1">Labels</Typography>
            {labels.map((label) => (
              <FormControlLabel
                key={label.id}
                control={
                  <Checkbox
                    defaultChecked={todo.labels.some(
                      (todoLabel) => todoLabel.id === label.id
                    )}
                  />
                }
                label={label.name}
                onChange={() =>
                  setEditLabels((prev) => toggleLabels(prev, label))
                }
              />
            ))}
          </Stack>
        </Box>
      </Modal>
    </Card>
  );
});

export default TodoItem;
