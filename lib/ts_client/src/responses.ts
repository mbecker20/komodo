import { CreateLocalUserResponse, ExchangeForJwtResponse, GetLoginOptionsResponse, LoginLocalUserResponse, LoginWithSecretResponse } from "./types";

export type AuthResponses = {
  GetLoginOptions: GetLoginOptionsResponse;
  CreateLocalUser: CreateLocalUserResponse;
	LoginLocalUser: LoginLocalUserResponse;
	ExchangeForJwt: ExchangeForJwtResponse;
	LoginWithSecret: LoginWithSecretResponse;
};

export type ApiResponses = {
}
