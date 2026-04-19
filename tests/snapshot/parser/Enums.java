enum Enums {
    ;


    enum E0 {
        A(1) {
            int x = 1;
        },
        B,
        C(3),
        D {

        },
        ;

        E0(int i) {
        }

        E0() {

        }

        void method() {
        }
    }

    enum E1 {
        ,
    }
    enum E7 {
        ,;
    }

    enum E8 {
        ,;
        int field;
    }

    enum E2 {
        A, B, C
    }

    enum E3 {
        A, B, C,
    }

    enum E4 {
        A;
    }

    enum E5 {
        ;
    }

    enum E6 {

    }
}