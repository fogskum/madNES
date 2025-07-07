use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StatusFlag: u8 {
        const Carry = 1 << 0;
        const Zero = 1 << 1;
        const InterruptDisable = 1 << 2;
        const Decimal = 1 << 3;
        const Break = 1 << 4;
        const Unused = 1 << 5;
        const Overflow = 1 << 6;
        const Negative = 1 << 7;
    }
}
