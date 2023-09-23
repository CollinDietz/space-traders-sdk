#[macro_use]
mod utils;

mod tests {
    mod contract {
        use space_traders_api::contract::*;

        #[test]
        fn should_be_deserializable() {
            let json_str = r#"
         {
            "id": "clmvdg7k7dap9s60coav75shp",
            "factionSymbol": "COSMIC",
            "type": "PROCUREMENT",
            "terms": {
               "deadline": "2023-09-30T01:48:20.149Z",
               "payment": {
                  "onAccepted": 23229,
                  "onFulfilled": 113742
               },
               "deliver": [
                  {
                     "tradeSymbol": "ALUMINUM_ORE",
                     "destinationSymbol": "X1-GM20-97222X",
                     "unitsRequired": 8900,
                     "unitsFulfilled": 0
                  }
               ]
            },
            "accepted": false,
            "fulfilled": false,
            "expiration": "2023-09-24T01:48:20.149Z",
            "deadlineToAccept": "2023-09-24T01:48:20.149Z"
         }
         "#;

            let actual: Contract = serde_json::from_str(json_str).unwrap();

            let expected = Contract {
                id: string!("clmvdg7k7dap9s60coav75shp"),
                faction: space_traders_api::faction::Factions::Cosmic,
                contract_type: ContractType::Procurement,
                accepted: false,
                fulfilled: false,
                _deprecated: string!("2023-09-24T01:48:20.149Z"),
                deadline_to_accept: Some(string!("2023-09-24T01:48:20.149Z")),
                terms: Terms {
                    deadline: string!("2023-09-30T01:48:20.149Z"),
                    payment: Payment {
                        on_accepted: 23229,
                        on_fulfilled: 113742,
                    },
                    deliver: Some(vec![Deliver {
                        trade_symbol: string!("ALUMINUM_ORE"),
                        destination_symbol: string!("X1-GM20-97222X"),
                        units_required: 8900,
                        units_fulfilled: 0,
                    }]),
                },
            };

            assert_eq!(expected, actual);
        }
    }
}
