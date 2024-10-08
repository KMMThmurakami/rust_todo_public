CREATE TABLE labels
(
    id   SERIAL PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE todo_labels
(
    id       SERIAL PRIMARY KEY,
    todo_id  INTEGER NOT NULL REFERENCES todos (id) ON DELETE CASCADE DEFERRABLE INITIALLY DEFERRED,
    label_id INTEGER NOT NULL REFERENCES labels (id) DEFERRABLE INITIALLY DEFERRED
);
