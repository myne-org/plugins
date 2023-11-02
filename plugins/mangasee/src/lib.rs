use extism_pdk::*;

#[plugin_fn]
pub fn search(_: ()) -> FnResult<()> {
    Ok(())
}

#[cfg(test)]
mod tests {}
