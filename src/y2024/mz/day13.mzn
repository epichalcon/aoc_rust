int : Ax;
int : Ay;

int : Bx;
int : By;

int : Prizex;
int : Prizey;

int : Atok = 3;
int : Btok = 1;

var 0..100 : a;
var 0..100 : b;

constraint (Ax * a) + (Bx * b) = Prizex;
constraint (Ay * a) + (By * b) = Prizey;

solve minimize Atok * a + Btok * b;
