static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_dart_fn_deliver_output);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_wire_init_app);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_wire_minimal_adder);
    dummy_var ^= ((int64_t) (void*) get_dart_object);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    return dummy_var;
}
