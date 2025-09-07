use rand::{rng, Rng};

#[derive(Debug)]
#[repr(u8)]
pub enum Creature {
    Dragon,
    Phoenix,
    Griffin,
    Unicorn,
    Kraken,
    Basilisk,
    Hydra,
    Chimera,
    Minotaur,
    Centaur,
    Pegasus,
    Sphinx,
    Cerberus,
    Cyclops,
    Harpy,
    Siren,
    Mermaid,
    Leviathan,
    Golem,
    Yeti,
    Sasquatch,
    Wendigo,
    Thunderbird,
    Kitsune,
    Tengu,
    Oni,
    Naga,
    Garuda,
    Rakshasa,
    Banshee,
    Selkie,
    Dullahan,
}

fn main() {
    
    let mut rng = rng();
    let variant = rng.random_range(0..=31u8);
    let variant: Creature = unsafe { std::mem::transmute(variant) };

    let name = match variant {
        Creature::Dragon => "Dragon",
        Creature::Phoenix => "Phoenix",
        Creature::Griffin => "Griffin",
        Creature::Unicorn => "Unicorn",
        Creature::Kraken => "Kraken",
        Creature::Basilisk => "Basilisk",
        Creature::Hydra => "Hydra",
        Creature::Chimera => "Chimera",
        Creature::Minotaur => "Minotaur",
        Creature::Centaur => "Centaur",
        Creature::Pegasus => "Pegasus",
        Creature::Sphinx => "Sphinx",
        Creature::Cerberus => "Cerberus",
        Creature::Cyclops => "Cyclops",
        Creature::Harpy => "Harpy",
        Creature::Siren => "Siren",
        Creature::Mermaid => "Mermaid",
        Creature::Leviathan => "Leviathan",
        Creature::Golem => "Golem",
        Creature::Yeti => "Yeti",
        Creature::Sasquatch => "Sasquatch",
        Creature::Wendigo => "Wendigo",
        Creature::Thunderbird => "Thunderbird",
        Creature::Kitsune => "Kitsune",
        Creature::Tengu => "Tengu",
        Creature::Oni => "Oni",
        Creature::Naga => "Naga",
        Creature::Garuda => "Garuda",
        Creature::Rakshasa => "Rakshasa",
        Creature::Banshee => "Banshee",
        Creature::Selkie => "Selkie",
        Creature::Dullahan => "Dullahan"
    };

    println!("{}", name);
}