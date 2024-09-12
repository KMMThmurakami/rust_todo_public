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
import {
  Box,
  Stack,
  Typography,
  CircularProgress,
  ListSubheader,
} from "@mui/material";
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
import styles from "./style.module.css";

const TodoApp: FC = memo(() => {
  const [todos, setTodos] = useState<Todo[]>([]);
  const [labels, setLabels] = useState<Label[]>([]);
  const [filterLabelId, setFilterLabelId] = useState<number | null>(null);
  const [deleteError, setDeleteError] = useState<string | null>(null);
  const [loading, setLoading] = useState<boolean>(true); // ローディング状態を追加

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
      setDeleteError(`【${name}】 \nTODOとの連携を解除してください!!`);
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
      setLoading(true); // ローディング開始
      const todos = await getTodoItems();
      const labelResponse = await getLabelItems();
      setTodos(todos);
      setLabels(labelResponse);
      setLoading(false); // ローディング終了
    };
    fetchData();
  }, []);

  return (
    <>
      <Box className={styles.title}>
        <Typography variant="h1">Todo App</Typography>
      </Box>
      <Box className={styles.sideNav}>
        <ListSubheader>LABELS</ListSubheader>
        {loading ? ( // ローディング中にスピナーを表示
          <div className={styles.progress}>
            <CircularProgress size={40} />
          </div>
        ) : (
          <Suspense fallback={<div>Loading...</div>}>
            <SideNav
              labels={labels}
              onSelectLabel={onSelectLabel}
              filterLabelId={filterLabelId}
              onSubmitNewLabel={onSubmitNewLabel}
              onDeleteLabel={onDeleteLabel}
              deleteError={deleteError}
              onResetErrText={onResetErrText}
            />
          </Suspense>
        )}
      </Box>
      <Box className={styles.contents}>
        <Box className={styles.contents_box}>
          <Stack>
            <TodoForm onSubmit={onSubmit} labels={labels} />
            <div className={styles.contents_title}>
              <Typography variant="h2">LIST</Typography>
            </div>
            {loading ? ( // ローディング中にスピナーを表示
              <div className={styles.progress}>
                <CircularProgress size={40} />
              </div>
            ) : (
              <Suspense fallback={<div>Loading...</div>}>
                <TodoList
                  todos={dispTodo}
                  labels={labels}
                  onUpdate={onUpdate}
                  onDelete={onDelete}
                />
              </Suspense>
            )}
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
