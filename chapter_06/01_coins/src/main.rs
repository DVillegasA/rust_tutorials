#[derive(Debug)]
enum UsState {
    Alaska,
    Texas,
    Ohio
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        },
    }
}

fn count_coins_in_purse(coins: &[Coin]) -> u32 {
    let mut count = 0;

    for coin in coins {
        if let Coin::Quarter(state) = coin {
            println!("State quarter from {:?}", state);
        } else {
            count += 1;
        }
    }

    count
}

fn main() {
    let coin_purse = [
        Coin::Penny,
        Coin::Nickel,
        Coin::Nickel,
        Coin::Nickel,
        Coin::Dime,
        Coin::Quarter(UsState::Alaska),
        Coin::Quarter(UsState::Ohio),
        Coin::Quarter(UsState::Texas),
    ];

    let mut total = 0;

    for coin in &coin_purse {
        total += value_in_cents(coin);
    }

    println!("Our coin purse has in total {total} cents");

    println!("The total amount of non-quarters in our purse is {}", count_coins_in_purse(&coin_purse));

}
