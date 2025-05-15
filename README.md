# CTEXT_DECODER

Is a simple library whose sole purpose is to decode x11's COMPOUND_TEXT type into UTF-8. 
It has a single function: decode_with_replacement, which will attempt to decode a COMPUND_TEXT and if it finds any weird escape sequences or 
strange characters given the current state of the encoding, it will add the Unicode Replacement Character and continue decoding. It
returns a DecodeWithReplacementResult which contains the decoded string and a boolean indicating whether or not any replacement characters were added.

Since the default encoding for COMPOUND_TEXT is latin-1 and x11's STRING type is latin-1. This crate can also be used to decode x11's STRING type.