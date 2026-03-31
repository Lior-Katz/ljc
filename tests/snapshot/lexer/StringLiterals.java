class StringLiterals {

    void test() {

        // --- Basic strings ---
        String s1 = "abc";
        String s2 = "";
        String s3 = "with spaces";
        String s4 = "123";

        // --- Escape sequences ---
        String e1 = "line\nbreak";
        String e2 = "tab\tchar";
        String e3 = "backspace\btest";
        String e4 = "carriage\rreturn";
        String e5 = "formfeed\fhere";

        String e6 = "quote: \"";
        String e7 = "apostrophe: \'";   // allowed but not required
        String e8 = "backslash: \\";
//
//         // --- Unicode escapes (processed before lexing) ---
//         String u1 = "A: \u0041";
//         String u2 = "alpha: \u03B1";
//         String u3 = "CJK: \u4F60";

        // --- Octal escapes ---
        String o1 = "\7";
        String o2 = "\79";
        String o3 = "\123";
        String o4 = "\12a";
        String o5 = "\1111"
        String o6 = "\000";
        String o7 = "\0001";

//         // --- Mixed content ---
//         String m1 = "mix\n\t\u0041\\\"";
        String m2 = "text with numbers 123 and symbols !@#";

        // --- Important: no actual line terminators inside literals ---
        String valid = "first\nsecond";  // escape, not a real newline
    }
}