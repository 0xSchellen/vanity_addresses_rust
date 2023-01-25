mod check;
// mod load;

fn main() {
    // load::load();
    //check::check("1", 370_000_000, 1_000_000_000); // 1_000_000_000
    //check::check("2", 1_223_000_000, 2_000_000_000);
    //check::check("3", 2_222_000_000, 3_000_000_000);
    check::check("4", 3_116_000_000, 4_000_000_000);
    ////check::check("5", 4_294_000_000, 4_294_967_295);

}

// cargo build --release
// ./target/release/van_redis_determ_32_all

