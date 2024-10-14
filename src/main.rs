struct Gadget {
    name: String,
}

struct Player {
    name: String,
    score: u32,
}

// Opgave 1: Ownership med gadgets
fn ownership_example() {
    let gadget = Gadget { name: String::from("iPhone") };
    let owner1 = gadget; // owner1 ejer gadgeten
    let owner2 = owner1; // ownership overføres til owner2

    // Denne linje vil give en fejl, fordi owner1 ikke længere har ejerskab:
    // println!("{} er hos owner1", owner1.name);

    println!("{} er hos owner2", owner2.name); // Dette virker fint
}

// Opgave 2: Immutable borrowing i et multiplayer-spil
fn borrowing_example() {
    let player = Player { name: String::from("Alice"), score: 50 };
    show_player_data(&player);

    // Forsøg at ændre spillerens score her (vil give en fejl):
    // player.score += 10;
}

fn show_player_data(player: &Player) {
    println!("{} har en score på {}", player.name, player.score);
}

// Opgave 3: Mutable borrowing i et inventar-system
fn mutable_borrowing_example() {
    let mut inventory = vec!["iPhone", "MacBook"];
    add_gadget(&mut inventory);

    // Denne linje vil give en fejl, fordi vi forsøger at låne immutabelt efter mutable lån:
    // println!("Gadgets i lageret: {:?}", inventory);
}

fn add_gadget(inventory: &mut Vec<&str>) {
    inventory.push("iPad");
    println!("Ny gadget tilføjet. Nu: {:?}", inventory);
}

// Opgave 4: Lifetimes i en besked-app
fn lifetimes_example() {
    let sender = String::from("Bob");
    let message = send_message(&sender, "Hej, Rust!");
    println!("Besked fra {}: {}", sender, message);
}

fn send_message<'a>(sender: &'a str, content: &'a str) -> &'a str {
    content
}

fn main() {
    println!("Opgave 1: Ownership med gadgets");
    ownership_example();
    println!("\nOpgave 2: Immutable borrowing i et multiplayer-spil");
    borrowing_example();
    println!("\nOpgave 3: Mutable borrowing i et inventar-system");
    mutable_borrowing_example();
    println!("\nOpgave 4: Lifetimes i en besked-app");
    lifetimes_example();
}
