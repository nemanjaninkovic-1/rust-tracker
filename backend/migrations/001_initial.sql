-- Initial database schema for RustTracker
-- Create custom enum types
CREATE TYPE task_status AS ENUM ('Todo', 'InProgress', 'Completed', 'Backlog');
CREATE TYPE task_priority AS ENUM ('Low', 'Medium', 'High', 'Urgent');

-- Create tasks table
CREATE TABLE tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    description TEXT,
    status task_status NOT NULL DEFAULT 'Todo',
    priority task_priority NOT NULL DEFAULT 'Medium',
    due_date TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create indexes for better query performance
CREATE INDEX idx_tasks_status ON tasks(status);
CREATE INDEX idx_tasks_priority ON tasks(priority);
CREATE INDEX idx_tasks_due_date ON tasks(due_date);
CREATE INDEX idx_tasks_created_at ON tasks(created_at);

-- Create trigger function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create trigger to automatically update updated_at timestamp
CREATE TRIGGER update_tasks_updated_at
    BEFORE UPDATE ON tasks
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Insert some sample data for testing
INSERT INTO tasks (title, description, status, priority, due_date) VALUES
('Setup development environment', 'Install Rust, Docker, and configure the development workspace', 'Completed', 'High', NOW() - INTERVAL '2 days'),
('Implement task creation API', 'Create REST endpoint for adding new tasks', 'Completed', 'High', NOW() - INTERVAL '1 day'),
('Add drag and drop functionality', 'Implement drag and drop for task status changes', 'InProgress', 'Medium', NOW() + INTERVAL '2 days'),
('Design user interface', 'Create modern and responsive UI with Tailwind CSS', 'InProgress', 'Medium', NOW() + INTERVAL '3 days'),
('Write comprehensive tests', 'Add unit and integration tests for all components', 'Todo', 'High', NOW() + INTERVAL '5 days'),
('Add task filtering', 'Implement filtering by status, priority, and due date', 'Todo', 'Low', NOW() + INTERVAL '7 days'),
('Performance optimization', 'Optimize database queries and frontend rendering', 'Backlog', 'Medium', NOW() + INTERVAL '14 days'),
('Add user authentication', 'Implement user accounts and authentication system', 'Backlog', 'Low', NOW() + INTERVAL '21 days');
