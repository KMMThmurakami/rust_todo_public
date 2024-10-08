import { useEffect, useState, FC } from "react";
import "modern-css-reset";
import { ThemeProvider, createTheme } from "@mui/material/styles";
import { Box, Stack, Typography } from "@mui/material";
import {
  Label,
  NewTodoPayload,
  Todo,
  NewLabelPayload,
  UpdateTodoPayload,
} from "./types/todo";
import TodoForm from "./components/TodoForm";
import TodoList from "./components/TodoList";
import SideNav from "./components/SideNav";
import {
  addTodoItem,
  getTodoItems,
  updateTodoItem,
  deleteTodoItem,
} from "./lib/api/todo";
import { addLabelItem, deleteLabelItem, getLabelItems } from "./lib/api/label";

const TodoApp: FC = () => {
  const [todos, setTodos] = useState<Todo[]>([]);
  const [labels, setLabels] = useState<Label[]>([]);
  const [filterLabelId, setFilterLabelId] = useState<number | null>(null);

  const [deleteError, setDeleteError] = useState<string | null>(null);

  const onSubmit = async (payload: NewTodoPayload) => {
    if (!payload.text) return;

    await addTodoItem(payload);
    // APIより再度Todo配列を取得
    const todos = await getTodoItems();
    setTodos(todos);
  };

  const onUpdate = async (updateTodo: UpdateTodoPayload) => {
    await updateTodoItem(updateTodo);
    // APIより再度Todo配列を取得
    const todos = await getTodoItems();
    setTodos(todos);
  };

  const onDelete = async (id: number) => {
    await deleteTodoItem(id);
    // APIより再度Todo配列を取得
    const todos = await getTodoItems();
    setTodos(todos);
  };

  const onSelectLabel = (label: Label | null) => {
    setFilterLabelId(label?.id ?? null);
  };

  const onSubmitNewLabel = async (newLabel: NewLabelPayload) => {
    if (!labels.some((label) => label.name === newLabel.name)) {
      const res = await addLabelItem(newLabel);
      setLabels([...labels, res]);
    }
  };

  const onDeleteLabel = async (id: number, name: string) => {
    setDeleteError(null);
    try {
      await deleteLabelItem(id);
      setLabels((prev) => prev.filter((label) => label.id !== id));
    } catch (e) {
      setDeleteError(`【${name}】 TODOとの連携を解除してください!!`);
    }
  };

  const onResetErrText = () => {
    setDeleteError(null); // モーダルを開く前にエラーメッセージをリセット
  };

  const dispTodo = filterLabelId
    ? todos.filter((todo) =>
        todo.labels.some((label) => label.id === filterLabelId)
      )
    : todos;

  useEffect(() => {
    (async () => {
      const todos = await getTodoItems();
      setTodos(todos);
      const labelResponse = await getLabelItems();
      setLabels(labelResponse);
    })();
  }, []);

  return (
    <>
      <Box
        sx={{
          backgroundColor: "white",
          borderBottom: "1px solid gray",
          display: "flex",
          alignItems: "center",
          position: "fixed",
          top: 0,
          p: 2,
          width: "100%",
          height: 80,
          zIndex: 3,
        }}
      >
        <Typography variant="h1">Todo App</Typography>
      </Box>
      <Box
        sx={{
          backgroundColor: "white",
          borderRight: "1px solid gray",
          position: "fixed",
          height: "calc(100% - 80px)",
          width: "20%",
          zIndex: 2,
          top: 80,
          left: 0,
        }}
      >
        <SideNav
          labels={labels}
          onSelectLabel={onSelectLabel}
          filterLabelId={filterLabelId}
          onSubmitNewLabel={onSubmitNewLabel}
          onDeleteLabel={onDeleteLabel}
          deleteError={deleteError}
          onResetErrText={onResetErrText}
        />
      </Box>
      <Box
        sx={{
          display: "flex",
          justifyContent: "center",
          position: "fixed",
          p: 5,
          mt: 10,
          width: "80%",
          top: 0,
          left: "20%",
        }}
      >
        <Box maxWidth={700} width="100%">
          <Stack>
            <TodoForm onSubmit={onSubmit} labels={labels}></TodoForm>
            <TodoList
              todos={dispTodo}
              labels={labels}
              onUpdate={onUpdate}
              onDelete={onDelete}
            />
          </Stack>
        </Box>
      </Box>
    </>
  );
};

const theme = createTheme({
  typography: {
    h1: {
      fontSize: 30,
    },
    h2: {
      fontSize: 20,
    },
  },
});

const App: FC = () => {
  return (
    <ThemeProvider theme={theme}>
      <TodoApp />
    </ThemeProvider>
  );
};

export default App;
