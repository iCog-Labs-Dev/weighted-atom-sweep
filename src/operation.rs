use std::sync::Arc;

use crate::sweep::{AtomHeader, AtomPosition};

/// Trait for operations that transform atoms.
///
/// Implementations should:
/// - Use `#[instrument(skip(self, zipper), name = "operation.{operation_name}")]`
/// - Emit debug-level logs for the start and completion of transformations
/// - Emit trace-level logs for detailed transformation steps
///
/// # Example Implementation with Tracing
/// ```ignore
/// use tracing::instrument;
///
/// struct MyOperation;
///
/// impl Operation<MyAtom> for MyOperation {
///     fn name(&self) -> &str { "my_operation" }
///
///     #[instrument(skip(self, zipper), name = "operation.my_operation")]
///     fn transform(&self, zipper: Arc<AtomPosition>) {
///         debug!("starting transformation");
///         // ... transformation logic ...
///         debug!("transformation complete");
///     }
/// }
/// ```
pub trait Operation<H: AtomHeader> {
    fn name(&self) -> &str;
    fn transform(&self, zipper: Arc<AtomPosition>) -> ();
}

pub trait OperationObserver<H, O>
where
    H: AtomHeader,
    O: Operation<H>,
{
    fn subscribe(&mut self, observer: O);
    fn unsubscribe(&mut self, observer: O);
}
