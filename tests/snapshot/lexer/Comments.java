/*
 * Top-level block comment
 * spanning multiple lines
 * /* This looks like a nested comment but is NOT */


public class Main {

    public static void main(String[] args) {

        // Simple single-line comment

        int a = 1; // trailing comment

        int b = 2 /* inline block comment */ + 3;

        int c = /* comment before */ 4 + 5;

        int d = 6 + /* mid-expression
                       multi-line
                       block comment */ 7;

        /* Block comment before statement */ int e = 8;

        int f = 9; /* Block comment after statement */

        int g = 10 /* comment */ + /* another */ 11;

        // /* This is NOT a block comment, just text inside a line comment */

        /* A block comment containing line comment markers:
           // this should NOT start a real line comment
           // still inside block comment
        */

        String s1 = "This is not a // comment";
        String s2 = "This is not a /* comment */ either";

        char ch = '/'; // slash character
        char star = '*'; // star character

        /* Block comment ending immediately after start marker */ int h = 12;

        int i = 13; /**/ int j = 14; // empty block comment in between

        /*
         * Block comment with tricky ending:
         * *****/
        int k = 15;

        /*/ This is a tricky comment start (still a valid block comment) /*/
        int l = 16;

        /* Block comment with many stars *************** still ends at first */
        int m = 17;

        // Line comment with block opener /* should be ignored

        int n = 18; // comment with // multiple // slashes

        /* Multiline comment with mixed content:
           "string-like text"
           'char-like text'
           // fake line comment
           /* comment should end here -> */

        int o = 19;

    }
}
// Edge: comment at EOF (no newline at end of file)