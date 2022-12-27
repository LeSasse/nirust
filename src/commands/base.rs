

// For every command, the trait ExecutableCommand should be implemented by
// writing an associated method 'execute'
pub trait ExecutableCommand {
    fn execute(&self);
}


