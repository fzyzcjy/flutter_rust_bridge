# SSE Codec

This codec uses a simple serialization based approach.
For example, all fields of a struct are written to a byte buffer one by one.
The byte array is transferred across the language boundary,
and the other side decodes the fields from the buffer to reconstruct the object.
