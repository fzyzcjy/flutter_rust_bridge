# When third-party is modifiable

If the third-party crate is modifiable, i.e. it is written by you as well,
then this page discusses some tricks for such scenario.

## Writing attributes directly

There is no need to [override attributes](override-attributes) in this case.
Indeed, just follow [this guide](../../custom/attributes) and write down something like 
`/// flutter_rust_bridge:whatever_attribute` in the third-party crate.
