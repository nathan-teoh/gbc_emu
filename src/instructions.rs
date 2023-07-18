pub enum Instruction{
    ADD(ArithmeticTarget),
    ADC(ArithmeticTarget),
    SUB(ArithmeticTarget),
    SBC(ArithmeticTarget),
    AND(ArithmeticTarget),
    OR(ArithmeticTarget),
    XOR(ArithmeticTarget),
}

pub enum ArithmeticTarget{
    A,B,C,D,E,H,L,
}
