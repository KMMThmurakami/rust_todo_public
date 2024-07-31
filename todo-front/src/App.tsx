import { useEffect, useState, FC } from "react";
import "modern-css-reset";
import { ThemeProvider, createTheme } from "@mui/material/styles";
import { Box, Stack, Typography } from "@mui/material";
import { NewTodoPayload, Todo } from "./types/todo";
import TodoForm from "./components/TodoForm";
import TodoList from "./components/TodoList";
import { addTodoItem, getTodoItems, updateTodoItem } from "./lib/api/todo";

const TodoApp: FC = () => {
  const [todos, setTodos] = useState<Todo[]>([]);
  // const createId = () => todos.length + 1;

  const onSubmit = async (payload: NewTodoPayload) => {
    if (!payload.text) return;

    await addTodoItem(payload);
    // APIより再度Todo配列を取得
    const todos = await getTodoItems();
    setTodos(todos);
  };

  const onUpdate = async (updateTodo: Todo) => {
    await updateTodoItem(updateTodo);
    // APIより再度Todo配列を取得
    const todos = await getTodoItems();
    setTodos(todos);
  };

  useEffect(() => {
    (async () => {
      const todos = await getTodoItems();
      setTodos(todos);
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
          display: "flex",
          justifyContent: "center",
          position: "fixed",
          p: 5,
          mt: 10,
          width: "100%",
          top: 0,
        }}
      >
        <Box maxWidth={700} width="100%">
          <Stack>
            <TodoForm onSubmit={onSubmit}></TodoForm>
            <TodoList todos={todos} onUpdate={onUpdate} onDelete={() => {}} />
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
