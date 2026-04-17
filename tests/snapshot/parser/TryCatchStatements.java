class TryCatchStatements {
    void basic_try_catch() {
        try {
            int x = 1 / 0;
        } catch (ArithmeticException e) {
            e.getMessage();
        }
    }

    void multiple_catch() {
        try {
            Integer.parseInt("abc");
        } catch (NumberFormatException e) {
            e.getMessage();
        } catch (Exception e) {
            e.getMessage();
        }
    }

    void union_catch() {
        try {
            Integer.parseInt("abc");
        } catch (NumberFormatException | NullPointerException e) {
            e.getMessage();
        }
    }

    void try_catch_finally() {
        try {
            int x = 10 / 2;
        } catch (Exception e) {
            e.getMessage();
        } finally {
            System.gc();
        }
    }

    void try_finally() {
        int x = 0;
        try {
            x = 1;
        } finally {
            x++;
        }
    }

    void try_with_resources() {
        try (StringReader in = new StringReader("abc")) {
            in.read();
        } catch (IOException e) {
            e.getMessage();
        }
    }

    void multiple_resources() {
        try (StringReader in = new StringReader("abc"); StringWriter out = new StringWriter()) {
            out.write(in.read());
        } catch (IOException e) {
            e.getMessage();
        }
    }

    void try_with_resources_finally() {
        try (StringReader in = new StringReader("abc")) {
            in.read();
        } catch (IOException e) {
            e.getMessage();
        } finally {
            System.gc();
        }
    }

    void try_with_resources_only() {
        try (StringReader in = new StringReader("abc")) {
            int x = 1;
        }
    }

    void resource_local_variable() {
        StringReader in = new StringReader("abc");

        try (in) {
            int x = 1;
        }
    }

    void resource_field_access(Holder h) {
        try (h.scanner) {
            int x = 1;
        }
    }

    void mixed_resources(Holder h) {
        Scanner in = new Scanner("abc");

        try (in; h.scanner; StringWriter out = new StringWriter()) {
            out.write(1);
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }

    void throw_in_try_catch() {
        try {
            throw new RuntimeException("error");
        } catch (RuntimeException e) {
            throw e;
        }
    }

    void throw_in_finally() {
        try {
            int x = 1;
        } finally {
            throw new RuntimeException();
        }
    }

    static class Holder {
        final Scanner scanner = new Scanner("abc");
    }
}