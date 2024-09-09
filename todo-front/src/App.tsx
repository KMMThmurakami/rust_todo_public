import {
  useEffect,
  useState,
  FC,
  memo,
  useMemo,
  lazy,
  Suspense,
  useCallback,
} from "react";
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
const TodoForm = lazy(() => import("./components/TodoForm"));
const TodoList = lazy(() => import("./components/TodoList"));
import SideNav from "./components/SideNav";
import {
  addTodoItem,
  getTodoItems,
  updateTodoItem,
  deleteTodoItem,
} from "./lib/api/todo";
import { addLabelItem, deleteLabelItem, getLabelItems } from "./lib/api/label";

const TodoApp: FC = memo(() => {
  const [todos, setTodos] = useState<Todo[]>([]);
  const [labels, setLabels] = useState<Label[]>([]);
  const [filterLabelId, setFilterLabelId] = useState<number | null>(null);
  const [deleteError, setDeleteError] = useState<string | null>(null);

  const onSubmit = async (payload: NewTodoPayload) => {
    if (!payload.text) return;
    const newTodo = await addTodoItem(payload);
    setTodos((prevTodos) => [...prevTodos, newTodo]);
  };

  const onUpdate = useCallback(async (updateTodo: UpdateTodoPayload) => {
    await updateTodoItem(updateTodo);
    setTodos(await getTodoItems());
  }, []);

  const onDelete = useCallback(async (id: number) => {
    await deleteTodoItem(id);
    setTodos(await getTodoItems());
  }, []);

  const onSelectLabel = useCallback((label: Label | null) => {
    setFilterLabelId(label?.id ?? null);
  }, []);

  const onSubmitNewLabel = useCallback(
    async (newLabel: NewLabelPayload) => {
      if (!labels.some((label) => label.name === newLabel.name)) {
        const res = await addLabelItem(newLabel);
        setLabels((prevLabels) => [...prevLabels, res]);
      }
    },
    [labels]
  );

  const onDeleteLabel = useCallback(async (id: number, name: string) => {
    setDeleteError(null);
    try {
      await deleteLabelItem(id);
      setLabels((prev) => prev.filter((label) => label.id !== id));
    } catch (e) {
      setDeleteError(`【${name}】 TODOとの連携を解除してください!!`);
    }
  }, []);

  const onResetErrText = useCallback(() => {
    setDeleteError(null);
  }, []);

  const dispTodo = useMemo(() => {
    return filterLabelId
      ? todos.filter((todo) =>
          todo.labels.some((label) => label.id === filterLabelId)
        )
      : todos;
  }, [filterLabelId, todos]);

  useEffect(() => {
    const fetchData = async () => {
      const todos = await getTodoItems();
      const labelResponse = await getLabelItems();
      setTodos(todos);
      setLabels(labelResponse);
    };
    fetchData();
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
            <Suspense fallback={<div>Loading...</div>}>
              <TodoForm onSubmit={onSubmit} labels={labels} />
              <TodoList
                todos={dispTodo}
                labels={labels}
                onUpdate={onUpdate}
                onDelete={onDelete}
              />
            </Suspense>
          </Stack>
        </Box>
      </Box>
    </>
  );
});

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
