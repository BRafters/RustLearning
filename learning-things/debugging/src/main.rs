fn main() {
    // All types which want to use std::fmt formatting traits require an implementation to be printable
    // Auto implementations are only provided for types such as in the std library
    // Others must be manually implemented somehow

    // The fmt::Debug trait makes this straightforward, all types can derive (auto create) the fmt::Debug implementation
    // Not true for fmt::Display which must be manually implemented

    // This struct cant pe printed with either fmt::Display or fmt::Debug
    struct UnPrintable(i32);

    // The 'derive' attribute auto creates the implementation to make this struct print with fmt::Debug
    #[derive(Debug)]
    struct DebugPrintable(i32);

    #[derive(Debug)]
    struct Structure(i32);

    // Put Structure in struct Deep. Also make it printable
    #[derive(Debug)]
    struct Deep(Structure);

    // All std library types are auto printable with {:?}
    fn main() {
        // Printing with {:?} is similar to with {}
        println!("{:?} months in a year", 12);
        println!("{1:?} {0:?} is the {actor:?} name.", "Slater", "Christian", actor="actor's");

        // Structure is printable
        println!("Now {:?} will print!", Structure(3));

        // No control on how the results will look with 'derive'
        println!("Now {:?} will print!", Deep(Structure(7)));

        // Rust provides pretty printing with {:?}
        println!("Now {:#?} will print!", Deep(Structure(3)));
    }

    main();
}
