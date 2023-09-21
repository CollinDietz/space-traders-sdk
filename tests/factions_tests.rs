mod tests {
    use space_traders_api::factions::Factions;

    static CASES: [(Factions, &'static str); 19] = [
        (Factions::Cosmic, "COSMIC"),
        (Factions::Void, "VOID"),
        (Factions::Galactic, "GALACTIC"),
        (Factions::Quantum, "QUANTUM"),
        (Factions::Dominion, "DOMINION"),
        (Factions::Astro, "ASTRO"),
        (Factions::Corsairs, "CORSAIRS"),
        (Factions::Obsidian, "OBSIDIAN"),
        (Factions::Aegis, "AEGIS"),
        (Factions::United, "UNITED"),
        (Factions::Solitary, "SOLITARY"),
        (Factions::Cobalt, "COBALT"),
        (Factions::Omega, "OMEGA"),
        (Factions::Echo, "ECHO"),
        (Factions::Lords, "LORDS"),
        (Factions::Cult, "CULT"),
        (Factions::Ancients, "ANCIENTS"),
        (Factions::Shadow, "SHADOW"),
        (Factions::Ethereal, "ETHEREAL"),
    ];

    #[test]
    fn should_be_convertible_to_string() {
        CASES.iter().for_each(|(faction, faction_string)| {
            assert_eq!(faction.to_string(), *faction_string)
        });
    }

    #[test]
    fn should_be_convertible_from_string() {
        CASES.iter().for_each(|(faction, faction_string)| {
            assert_eq!(*faction, faction_string.parse::<Factions>().unwrap())
        });
    }
}
