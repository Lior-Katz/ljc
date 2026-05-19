class Test {
    char a = ' /* this '; // should count as a closing quote and the error should be for too many characters
}