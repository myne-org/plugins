# built-ins

This directory hosts built-in plugins. This are used directly within the app as cargo crates and are faster than the general plugins that are compiled to wasm and used via extism. 

Since these plugins cannot be changed once shipped, they must be only made for sites/provides that ensure some form of stability (like mangadex with its stable public api) and won't break (scrapers may be break suddenly if for some reason the site's underlying code changes).