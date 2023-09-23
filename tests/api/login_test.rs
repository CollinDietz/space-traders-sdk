mod testss {
    use space_traders_api::api::Api;
    use space_traders_api::faction::Factions;

    #[tokio::test]
    async fn should_be_able_to_register() {
        let api = Api::new();
        let result = api
            .register("SomeGuy".to_string(), Factions::Cosmic)
            .await
            .unwrap();
        println!("{}", result);
    }
}
