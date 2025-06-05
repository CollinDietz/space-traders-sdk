use std::sync::Arc;

use derivative::Derivative;
use reqwest::{Response, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
use serde_derive::Deserialize;
use serde_repr::Deserialize_repr;

const REAL_SERVER: &'static str = "https://api.spacetraders.io/v2";

#[derive(Debug, Deserialize_repr, PartialEq)]
#[repr(u16)]
pub enum ErrorCode {
    ResponseSerializationError = 3000,
    UnprocessableInputError = 3001,
    AllErrorHandlersFailedError = 3002,
    SystemStatusMaintenanceError = 3100,
    ResetError = 3200,
    CooldownConflictError = 4000,
    WaypointNoAccessError = 4001,
    TokenEmptyError = 4100,
    TokenMissingSubjectError = 4101,
    TokenInvalidSubjectError = 4102,
    MissingTokenRequestError = 4103,
    InvalidTokenRequestError = 4104,
    InvalidTokenSubjectError = 4105,
    AccountNotExistsError = 4106,
    AgentNotExistsError = 4107,
    AccountHasNoAgentError = 4108,
    TokenInvalidVersionError = 4109,
    RegisterAgentSymbolReservedError = 4110,
    RegisterAgentConflictSymbolError = 4111,
    RegisterAgentNoStartingLocationsError = 4112,
    TokenResetDateMismatchError = 4113,
    InvalidAccountRoleError = 4114,
    InvalidTokenError = 4115,
    MissingAccountTokenRequest = 4116,
    NavigateInTransitError = 4200,
    NavigateInvalidDestinationError = 4201,
    NavigateOutsideSystemError = 4202,
    NavigateInsufficientFuelError = 4203,
    NavigateSameDestinationError = 4204,
    ShipExtractInvalidWaypointError = 4205,
    ShipExtractPermissionError = 4206,
    ShipInTransitError = 4214,
    ShipMissingSensorArraysError = 4215,
    PurchaseShipCreditsError = 4216,
    ShipCargoExceedsLimitError = 4217,
    ShipCargoMissingError = 4218,
    ShipCargoUnitCountError = 4219,
    ShipSurveyVerificationError = 4220,
    ShipSurveyExpirationError = 4221,
    ShipSurveyWaypointTypeError = 4222,
    ShipSurveyOrbitError = 4223,
    ShipSurveyExhaustedError = 4224,
    ShipCargoFullError = 4228,
    WaypointChartedError = 4230,
    ShipTransferShipNotFound = 4231,
    ShipTransferAgentConflict = 4232,
    ShipTransferSameShipConflict = 4233,
    ShipTransferLocationConflict = 4234,
    WarpInsideSystemError = 4235,
    ShipNotInOrbitError = 4236,
    ShipInvalidRefineryGoodError = 4237,
    ShipInvalidRefineryTypeError = 4238,
    ShipMissingRefineryError = 4239,
    ShipMissingSurveyorError = 4240,
    ShipMissingWarpDriveError = 4241,
    ShipMissingMineralProcessorError = 4242,
    ShipMissingMiningLasersError = 4243,
    ShipNotDockedError = 4244,
    PurchaseShipNotPresentError = 4245,
    ShipMountNoShipyardError = 4246,
    ShipMissingMountError = 4247,
    ShipMountInsufficientCreditsError = 4248,
    ShipMissingPowerError = 4249,
    ShipMissingSlotsError = 4250,
    ShipMissingMountsError = 4251,
    ShipMissingCrewError = 4252,
    ShipExtractDestabilizedError = 4253,
    ShipJumpInvalidOriginError = 4254,
    ShipJumpInvalidWaypointError = 4255,
    ShipJumpOriginUnderConstructionError = 4256,
    ShipMissingGasProcessorError = 4257,
    ShipMissingGasSiphonsError = 4258,
    ShipSiphonInvalidWaypointError = 4259,
    ShipSiphonPermissionError = 4260,
    WaypointNoYieldError = 4261,
    ShipJumpDestinationUnderConstructionError = 4262,
    ShipScrapInvalidTraitError = 4263,
    ShipRepairInvalidTraitError = 4264,
    AgentInsufficientCreditsError = 4265,
    ShipModuleNoShipyardError = 4266,
    ShipModuleNotInstalledError = 4267,
    ShipModuleInsufficientCreditsError = 4268,
    CantSlowDownWhileInTransitError = 4269,
    ShipExtractInvalidSurveyLocationError = 4270,
    ShipTransferDockedOrbitConflict = 4271,
    AcceptContractNotAuthorizedError = 4500,
    AcceptContractConflictError = 4501,
    FulfillContractDeliveryError = 4502,
    ContractDeadlineError = 4503,
    ContractFulfilledError = 4504,
    ContractNotAcceptedError = 4505,
    ContractNotAuthorizedError = 4506,
    ShipDeliverTermsError = 4508,
    ShipDeliverFulfilledError = 4509,
    ShipDeliverInvalidLocationError = 4510,
    ExistingContractError = 4511,
    MarketTradeInsufficientCreditsError = 4600,
    MarketTradeNoPurchaseError = 4601,
    MarketTradeNotSoldError = 4602,
    MarketTradeUnitLimitError = 4604,
    ShipNotAvailableForPurchaseError = 4605,
    WaypointNoFactionError = 4700,
    ConstructionMaterialNotRequired = 4800,
    ConstructionMaterialFulfilled = 4801,
    ShipConstructionInvalidLocationError = 4802,
    UnsupportedMediaTypeError = 5000,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct ErrorData {
    pub message: String,
    pub code: ErrorCode,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Error {
    pub error: ErrorData,
}

#[derive(Debug, Derivative, Clone)]
#[derivative(PartialEq)]
pub struct SpaceTradersClient {
    #[derivative(PartialEq = "ignore")]
    client: Arc<reqwest::Client>,
    pub url: String,
    token: String,
}

impl SpaceTradersClient {
    pub fn new(token: &str) -> Self {
        SpaceTradersClient {
            client: Arc::new(reqwest::Client::new()),
            url: REAL_SERVER.to_string(),
            token: token.to_string(),
        }
    }

    pub fn clone_with_token(client: &SpaceTradersClient, new_token: &str) -> Self {
        SpaceTradersClient {
            token: new_token.to_string(),
            ..client.clone()
        }
    }

    pub fn with_url(url: &str, token: &str) -> Self {
        SpaceTradersClient {
            client: Arc::new(reqwest::Client::new()),
            url: url.to_string(),
            token: token.to_string(),
        }
    }

    async fn _get(
        client: &reqwest::Client,
        url: &str,
        endpoint: &str,
        token: &str,
    ) -> Result<Response, reqwest::Error> {
        client
            .get(&format!("{}/{}", url, endpoint))
            .bearer_auth(token)
            .send()
            .await
    }

    async fn internal_post<T: Serialize + ?Sized, R: DeserializeOwned>(
        client: &reqwest::Client,
        url: &str,
        endpoint: &str,
        token: &str,
        body: Option<&T>,
        success_status: StatusCode,
    ) -> Result<R, reqwest::Error> {
        let mut request = client
            .post(&format!("{}/{}", url, endpoint))
            .bearer_auth(token)
            .header("Accept", "application/json");

        if let Some(body) = body {
            request = request.json(body);
        };

        let result = request.send().await;

        match result {
            Ok(response) => {
                if response.status() == success_status {
                    Ok(response.json().await?)
                } else {
                    todo!()
                }
            }
            Err(_error) => {
                todo!()
            }
        }
    }

    pub async fn post<R: DeserializeOwned>(
        &self,
        endpoint: &str,
        success_status: StatusCode,
    ) -> Result<R, reqwest::Error> {
        SpaceTradersClient::internal_post::<serde_json::Value, R>(
            &self.client,
            &self.url,
            endpoint,
            &self.token,
            None,
            success_status,
        )
        .await
    }

    pub async fn post_with_body<T: Serialize + ?Sized, R: DeserializeOwned>(
        &self,
        endpoint: &str,
        body: &T,
        success_status: StatusCode,
    ) -> Result<R, reqwest::Error> {
        SpaceTradersClient::internal_post(
            &self.client,
            &self.url,
            endpoint,
            &self.token,
            Some(body),
            success_status,
        )
        .await
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{
        space_traders_client::{Error, ErrorCode, ErrorData},
        string,
    };

    #[test]
    fn missing_token_error_should_be_deserializable() {
        let json_str = r#"
        {
            "error": {
                "code": 4103,
                "message": "Missing Bearer token in the request. Did you confirm sending the 'Bearer {token}' as the authorization header?",
                "data": {},
                "requestId": "0197408d-a4ad-7351-acef-c3c42a4ffd06"
            }
        }"#;

        let actual: Error = serde_json::from_str(json_str).unwrap();

        let expected =
        Error {
            error: ErrorData {
                message: string!("Missing Bearer token in the request. Did you confirm sending the 'Bearer {token}' as the authorization header?"),
                code: ErrorCode::MissingTokenRequestError,
            }
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn cooldown_conflict_error_should_be_deserializable() {
        let json_str = r#"
        {
            "error": {
                "code": 4000,
                "message": "Ship action is still on cooldown for 47 second(s).",
                "data": {
                    "cooldown": {
                        "shipSymbol": "SHOOTTEST-3",
                        "totalSeconds": 70,
                        "remainingSeconds": 47,
                        "expiration": "2025-06-01T18:38:08.614Z"
                    }
                },
                "requestId": "01972cc8-b922-73f7-b929-49e43546ef15"
            }
        }"#;

        let actual: Error = serde_json::from_str(json_str).unwrap();

        let expected = Error {
            error: ErrorData {
                message: string!("Ship action is still on cooldown for 47 second(s)."),
                code: ErrorCode::CooldownConflictError,
            },
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn unprocessable_input_error_should_be_deserializable() {
        let json_str = r#"
        {
            "error": {
                "code": 3001,
                "message": "You specified a 'Content-Type' header of 'application/json', but the request body is an empty string (which can't be parsed as valid JSON). Send an empty object (e.g. {}) instead.",
                "requestId": "01974086-5ec7-777f-b735-1c0fb6e7442d"
            }
        }"#;

        let actual: Error = serde_json::from_str(json_str).unwrap();

        let expected = Error {
            error: ErrorData {
                message: string!("You specified a 'Content-Type' header of 'application/json', but the request body is an empty string (which can't be parsed as valid JSON). Send an empty object (e.g. {}) instead."),
                code: ErrorCode::UnprocessableInputError,
            },
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn unprocessable_input_error_with_data_should_be_deserializable() {
        let json_str = r#"
        {
            "error": {
                "code": 3001,
                "message": "The request could not be processed due to an invalid payload or application state.",
                "data": {
                "zodIssues": [
                    {
                    "code": "invalid_type",
                    "expected": "object",
                    "received": "string",
                    "path": [],
                    "message": "Expected object, received string"
                    }
                ],
                "originalError": {
                    "issues": [
                    {
                        "code": "invalid_type",
                        "expected": "object",
                        "received": "string",
                        "path": [],
                        "message": "Expected object, received string"
                    }
                    ],
                    "name": "ZodError"
                }
                },
                "requestId": "01974087-b835-714d-a5fa-08ead571d78c"
            }
        }"#;

        let actual: Error = serde_json::from_str(json_str).unwrap();

        let expected = Error {
            error: ErrorData {
                message: string!("The request could not be processed due to an invalid payload or application state."),
                code: ErrorCode::UnprocessableInputError,
            },
        };

        assert_eq!(expected, actual);
    }
}
// TODO: Add tests
