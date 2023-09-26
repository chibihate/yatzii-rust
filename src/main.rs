mod dices;

fn main() {
    let mut dice = dices::dices::Dice::origin();
    dice.roll();
    let mut dices = dices::dices::Dices::origin();
    for dice_iter in dices.get_value().iter_mut() {
        println!("{:?}", dice_iter);
    }
    dices.roll();
    println!("{:?}", dice);

    for dice_iter in dices.get_value().iter_mut() {
        println!("{:?}", dice_iter);
    }
}
