mod tests {
   use space_traders_api::factions::Factions;

   #[test]
   fn should_be_convertible_to_string()
   {
      assert_eq!(Factions::Cosmic.to_string(), "COSMIC");
      assert_eq!(Factions::Void.to_string(), "VOID");
      assert_eq!(Factions::Galactic.to_string(), "GALACTIC");
      assert_eq!(Factions::Quantum.to_string(), "QUANTUM");
      assert_eq!(Factions::Dominion.to_string(), "DOMINION");
      assert_eq!(Factions::Astro.to_string(), "ASTRO");
      assert_eq!(Factions::Corsairs.to_string(), "CORSAIRS");
      assert_eq!(Factions::Obsidian.to_string(), "OBSIDIAN");
      assert_eq!(Factions::Aegis.to_string(), "AEGIS");
      assert_eq!(Factions::United.to_string(), "UNITED");
      assert_eq!(Factions::Solitary.to_string(), "SOLITARY");
      assert_eq!(Factions::Cobalt.to_string(), "COBALT");
      assert_eq!(Factions::Omega.to_string(), "OMEGA");
      assert_eq!(Factions::Echo.to_string(), "ECHO");
      assert_eq!(Factions::Lords.to_string(), "LORDS");
      assert_eq!(Factions::Cult.to_string(), "CULT");
      assert_eq!(Factions::Ancients.to_string(), "ANCIENTS");
      assert_eq!(Factions::Shadow.to_string(), "SHADOW");
      assert_eq!(Factions::Ethereal.to_string(), "ETHEREAL");
   }

   #[test]
   fn should_be_convertible_from_string()
   {
      assert_eq!("COSMIC".parse::<Factions>().unwrap(), Factions::Cosmic);
      assert_eq!("VOID".parse::<Factions>().unwrap(), Factions::Void);
      assert_eq!("GALACTIC".parse::<Factions>().unwrap(), Factions::Galactic);
      assert_eq!("QUANTUM".parse::<Factions>().unwrap(), Factions::Quantum);
      assert_eq!("DOMINION".parse::<Factions>().unwrap(), Factions::Dominion);
      assert_eq!("ASTRO".parse::<Factions>().unwrap(), Factions::Astro);
      assert_eq!("CORSAIRS".parse::<Factions>().unwrap(), Factions::Corsairs);
      assert_eq!("OBSIDIAN".parse::<Factions>().unwrap(), Factions::Obsidian);
      assert_eq!("AEGIS".parse::<Factions>().unwrap(), Factions::Aegis);
      assert_eq!("UNITED".parse::<Factions>().unwrap(), Factions::United);
      assert_eq!("SOLITARY".parse::<Factions>().unwrap(), Factions::Solitary);
      assert_eq!("COBALT".parse::<Factions>().unwrap(), Factions::Cobalt);
      assert_eq!("OMEGA".parse::<Factions>().unwrap(), Factions::Omega);
      assert_eq!("ECHO".parse::<Factions>().unwrap(), Factions::Echo);
      assert_eq!("LORDS".parse::<Factions>().unwrap(), Factions::Lords);
      assert_eq!("CULT".parse::<Factions>().unwrap(), Factions::Cult);
      assert_eq!("ANCIENTS".parse::<Factions>().unwrap(), Factions::Ancients);
      assert_eq!("SHADOW".parse::<Factions>().unwrap(), Factions::Shadow);
      assert_eq!("ETHEREAL".parse::<Factions>().unwrap(), Factions::Ethereal); 
   }
}
