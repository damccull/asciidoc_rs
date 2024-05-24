pub mod telemetry;

trait TreeNode {}

pub struct DocumentTree {
    leaves: Vec<Box<dyn TreeNode>>,
}

#[cfg(test)]
mod tests {}
