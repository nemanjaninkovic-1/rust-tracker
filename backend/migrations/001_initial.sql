-- Create custom types for task status and category
CREATE TYPE task_status AS ENUM ('Todo', 'InProgress', 'Completed');
CREATE TYPE task_category AS ENUM ('Work', 'Personal', 'Shopping', 'Health', 'Other');

-- Create tasks table
CREATE TABLE tasks (
    id UUID PRIMARY KEY,
    title VARCHAR NOT NULL,
    description TEXT,
    status task_status NOT NULL DEFAULT 'Todo',
    category task_category NOT NULL DEFAULT 'Other',
    due_date TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);

-- Create indexes for better query performance
CREATE INDEX idx_tasks_status ON tasks(status);
CREATE INDEX idx_tasks_category ON tasks(category);
CREATE INDEX idx_tasks_due_date ON tasks(due_date);
CREATE INDEX idx_tasks_created_at ON tasks(created_at);
