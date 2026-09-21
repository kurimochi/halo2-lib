use crate::{utils::ScalarField, Context};

use super::SinglePhaseCoreManager;

/// Utility function to apply an operation to multiple [`Context`]s.
pub fn parallelize_core<F, T, R, FR>(
    builder: &mut SinglePhaseCoreManager<F>, // leaving `builder` for historical reasons, `pool` is a better name
    input: Vec<T>,
    f: FR,
) -> Vec<R>
where
    F: ScalarField,
    FR: Fn(&mut Context<F>, T) -> R,
{
    // to prevent concurrency issues with context id, we generate all the ids first
    let thread_count = builder.thread_count();
    let mut ctxs =
        (0..input.len()).map(|i| builder.new_context(thread_count + i)).collect::<Vec<_>>();
    let outputs: Vec<_> =
        input.into_iter().zip(ctxs.iter_mut()).map(|(input, ctx)| f(ctx, input)).collect();
    // we collect the new threads to ensure they are a FIXED order, otherwise the circuit will not be deterministic
    builder.threads.append(&mut ctxs);

    outputs
}
