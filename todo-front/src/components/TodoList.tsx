import { FC, memo, useCallback, useMemo } from "react";
import type { Label, Todo, UpdateTodoPayload } from "../types/todo";
import { Stack } from "@mui/material";
import TodoItem from "./TodoItem";

type Props = {
  todos: Todo[];
  labels: Label[];
  onUpdate: (todo: UpdateTodoPayload) => Promise<void>; // 非同期関数
  onDelete: (id: number) => Promise<void>; // 非同期関数
};

const TodoList: FC<Props> = memo(({ todos, labels, onUpdate, onDelete }) => {
  // onUpdateとonDeleteをuseCallbackでメモ化して再レンダリングを防止
  const handleUpdate = useCallback(
    async (todo: UpdateTodoPayload) => {
      await onUpdate(todo); // 非同期処理の実行
    },
    [onUpdate]
  );

  const handleDelete = useCallback(
    async (id: number) => {
      await onDelete(id); // 非同期処理の実行
    },
    [onDelete]
  );

  // ID順でソートされたtodosをメモ化
  const sortedTodos = useMemo(() => {
    return [...todos].sort((a, b) => a.id - b.id);
  }, [todos]);

  return (
    <Stack spacing={2}>
      {sortedTodos.map((todo) => (
        <TodoItem
          key={todo.id}
          todo={todo}
          onUpdate={handleUpdate}
          onDelete={handleDelete}
          labels={labels}
        />
      ))}
    </Stack>
  );
});

export default TodoList;
